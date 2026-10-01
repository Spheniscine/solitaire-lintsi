use std::time::Duration;

use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{components::LocalStorage, game::{Board, BoardPos, Card, DECK_SIZE, DepotRole, NUM_RANKS, RANKS, Skin, Suit}};

pub const ANIMATION_DURATION: Duration = Duration::from_millis(200);
pub type AnimationKey = u16;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ActionRecord {
    Move { pos1: BoardPos, pos2: BoardPos, rev: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ScreenState {
    #[default] Game, 
    Settings, Help,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub deal: Vec<Card>,
    #[serde(skip)]
    pub animation_key: AnimationKey, // used for syncing and to provide animator components with cycling keys
    pub history: Vec<ActionRecord>,
    pub undo_stack: Vec<usize>,
    pub already_won: bool,
    pub num_wins: i32,

    pub screen_state: ScreenState,

    pub allow_undo: bool,
    pub skin: Skin,
}

impl GameState {
    pub fn new_deal(rng: &mut impl Rng) -> Vec<Card> {
        let mut deck = Vec::with_capacity(DECK_SIZE);
        for rank in RANKS {
            for suit in Suit::iter() {
                deck.push(Card::Suited { rank, suit });
            }
        }

        deck.shuffle(rng);
        deck
    }

    pub fn init() -> Self {
        let mut res = Self {
            board: Board::empty(),
            deal: vec![],
            animation_key: 0,
            history: vec![],
            undo_stack: vec![],
            already_won: false,
            num_wins: 0,
            screen_state: ScreenState::Game,
            allow_undo: true,
            skin: Skin::default(),
        };

        res.new_game();
        res
    }

    pub fn new_game(&mut self) {
        let deal = Self::new_deal(&mut rand::rng());
        self.board = Board::from_deal(&deal);
        self.deal = deal;
        self.history.clear();
        self.undo_stack.clear();
        self.already_won = false;
        LocalStorage.save_game_state(&self);
    }

    pub fn is_busy(&self) -> bool {
        self.is_acting()
    }

    pub fn is_acting(&self) -> bool {
        !self.board.animation_acts.is_empty()
    }

    pub fn undo_possible(&self) -> bool {
        self.allow_undo && !self.undo_stack.is_empty()
    }

    fn do_move_raw(&mut self, pos1: BoardPos, pos2: BoardPos, rev: bool) {
        self.board.do_move(pos1, pos2, rev);
        self.history.push(ActionRecord::Move { pos1, pos2, rev });
    }

    fn can_stack(&self, back: Card, front: Card) -> bool {
        match (back, front) {
            (Card::Suited { rank: r1, .. }, Card::Suited { rank: r2, .. }) => {
                r1 == r2 + 1
            },
            _ => true,
        }
    }

    fn can_group(&self, back: Card, front: Card) -> bool {
        match (back, front) {
            (Card::Suited { rank: r1, suit: s1 }, Card::Suited { rank: r2, suit: s2 }) => {
                r1 == r2 + 1 && s1 == s2
            },
            _ => false,
        }
    }

    pub fn can_select(&self, pos: BoardPos) -> bool {
        let depot = pos.depot_index;
        let ord = pos.card_index;

        if ord >= self.board.depots[depot].len() {
            return false;
        }
        let slice = &self.board.depots[depot][ord..];

        let Some(role) = DepotRole::role(depot) else { return false };
        match role {
            DepotRole::Tableau => slice.windows(2).all(|w| self.can_group(w[0], w[1])),
            DepotRole::JokerPile => slice.len() == 1,
            DepotRole::Foundation => false,
        }
    }

    pub fn is_won(&self) -> bool {
        DepotRole::Foundation.range().all(|d| {
            self.board.depots[d].len() == NUM_RANKS
        })
    }

    pub fn is_over(&self) -> bool {
        self.is_won()
    }

    fn move_intent(&mut self, pos1: BoardPos, pos2: BoardPos) -> bool {
        if pos1.depot_index == pos2.depot_index { return false; }
        let depot1 = &self.board.depots[pos1.depot_index];
        let depot2 = &self.board.depots[pos2.depot_index];
        let num_moved = depot1.len() - pos1.card_index;
        if pos2.card_index != depot2.len() { return false; }

        let card = depot1[pos1.card_index];
        let Some(role) = DepotRole::role(pos2.depot_index) else { return false };
        let history_len = self.history.len();
        match role {
            DepotRole::Tableau => {
                let ok = depot2.last().is_none_or(|&c| self.can_stack(c, card));
                if !ok { return false; }
                self.do_move_raw(pos1, pos2, false);
            },
            DepotRole::JokerPile => {
                if num_moved != 1 || card != Card::Joker { return false; }
                self.do_move_raw(pos1, pos2, false);
            },
            DepotRole::Foundation => {
                let Some(&front_card) = depot1.last() else { return false };
                let ok = if let Some(&card) = depot2.last() {
                    self.can_group(front_card, card)
                } else {
                    matches!(front_card, Card::Suited { rank: 1, .. })
                };
                if !ok { return false; }
                self.do_move_raw(pos1, pos2, true);
            },
        }

        self.undo_stack.push(history_len);
        true
    }

    pub fn onclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }
        if self.is_over() { return; }

        if let Some(src) = self.board.selected {
            if pos == src { 
                self.board.selected = None; 
                return;
            }
            if src.depot_index == pos.depot_index && self.can_select(pos) {
                self.board.selected = Some(pos);
                return;
            }

            let dest = BoardPos { depot_index: pos.depot_index, card_index: pos.card_index.wrapping_add(1) };
            self.move_intent(src, dest);
        } else {
            if self.can_select(pos) {
                self.board.selected = Some(pos);
            }
        }
    }

    pub fn ondoubleclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }
        if self.is_over() { return; }
        if !self.can_select(pos) { return; } // needed, or illegal stacks can still be moved this way!

        let depot = &self.board.depots[pos.depot_index];
        let card = depot[pos.card_index];

        match card {
            Card::Suited { .. } => {
                for d in DepotRole::Foundation.range() {
                    let dest = self.board.top_pos(d);
                    if self.move_intent(pos, dest) {
                        return;
                    }
                }
            },
            Card::Joker => {
                self.move_intent(pos, self.board.top_pos(DepotRole::JokerPile.id(0)));
            },
        }
    }

    pub fn advance_animations(&mut self, key: AnimationKey) {
        if key != self.animation_key { return; }
        self.animation_key = self.animation_key.wrapping_add(1);
        
        self.board.advance_actions();

        if self.is_won() {
            if !self.already_won {
                self.num_wins += 1;
                self.already_won = true;
            }
        } else {
            // self.check_auto_moves();
        }

        if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }

    pub fn undo(&mut self) {
        if self.is_busy() || !self.undo_possible() { return; }
        let Some(target_len) = self.undo_stack.pop() else {return};
        while self.history.len() > target_len {
            let rec = self.history.pop().unwrap();
            match rec {
                ActionRecord::Move { pos1, pos2, rev } => {
                    self.board.do_move(pos2, pos1, rev)
                },
            }
            self.board.advance_actions(); // no animation, as repeated card moves on same card causes problems
        }
        LocalStorage.save_game_state(&self);
    }

    pub fn restart(&mut self) {
        if self.history.is_empty() || !self.undo_possible() { return; }
        self.board = Board::from_deal(&self.deal);
        self.history.clear();
        self.undo_stack.clear();
        LocalStorage.save_game_state(&self);
    }
}