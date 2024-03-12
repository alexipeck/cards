use enum_iterator::Sequence;
use peck_lib::crypto::generate_random_number;
use std::{
    collections::{HashMap, VecDeque},
    io,
};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Sequence, EnumIter)]
pub enum CardVariant {
    OneClub = 0,
    OneDiamond = 1,
    OneHeart = 2,
    OneSpade = 3,
    TwoClub = 4,
    TwoDiamond = 5,
    TwoHeart = 6,
    TwoSpade = 7,
    ThreeClub = 8,
    ThreeDiamond = 9,
    ThreeHeart = 10,
    ThreeSpade = 11,
    FourClub = 12,
    FourDiamond = 13,
    FourHeart = 14,
    FourSpade = 15,
    FiveClub = 16,
    FiveDiamond = 17,
    FiveHeart = 18,
    FiveSpade = 19,
    SixClub = 20,
    SixDiamond = 21,
    SixHeart = 22,
    SixSpade = 23,
    SevenClub = 24,
    SevenDiamond = 25,
    SevenHeart = 26,
    SevenSpade = 27,
    EightClub = 28,
    EightDiamond = 29,
    EightHeart = 30,
    EightSpade = 31,
    NineClub = 32,
    NineDiamond = 33,
    NineHeart = 34,
    NineSpade = 35,
    TenClub = 36,
    TenDiamond = 37,
    TenHeart = 38,
    TenSpade = 39,
    JackClub = 40,
    JackDiamond = 41,
    JackHeart = 42,
    JackSpade = 43,
    QueenClub = 44,
    QueenDiamond = 45,
    QueenHeart = 46,
    QueenSpade = 47,
    KingClub = 48,
    KingDiamond = 49,
    KingHeart = 50,
    KingSpade = 51,
    Joker = 52,
}

impl CardVariant {
    pub fn get_value(&self) -> u8 {
        match self {
            Self::Joker => 0,
            Self::OneClub | Self::OneDiamond | Self::OneHeart | Self::OneSpade => 1,
            Self::TwoClub | Self::TwoDiamond | Self::TwoHeart | Self::TwoSpade => 2,
            Self::ThreeClub | Self::ThreeDiamond | Self::ThreeHeart | Self::ThreeSpade => 3,
            Self::FourClub | Self::FourDiamond | Self::FourHeart | Self::FourSpade => 4,
            Self::FiveClub | Self::FiveDiamond | Self::FiveHeart | Self::FiveSpade => 5,
            Self::SixClub | Self::SixDiamond | Self::SixHeart | Self::SixSpade => 6,
            Self::SevenClub | Self::SevenDiamond | Self::SevenHeart | Self::SevenSpade => 7,
            Self::EightClub | Self::EightDiamond | Self::EightHeart | Self::EightSpade => 8,
            Self::NineClub | Self::NineDiamond | Self::NineHeart | Self::NineSpade => 9,
            Self::TenClub | Self::TenDiamond | Self::TenHeart | Self::TenSpade => 10,
            Self::JackClub | Self::JackDiamond | Self::JackHeart | Self::JackSpade => 11,
            Self::QueenClub | Self::QueenDiamond | Self::QueenHeart | Self::QueenSpade => 12,
            Self::KingClub | Self::KingDiamond | Self::KingHeart | Self::KingSpade => 13,
        }
    }
    pub fn get_rank(&self) -> Option<Rank> {
        match self {
            Self::Joker => None,
            Self::OneClub | Self::OneDiamond | Self::OneHeart | Self::OneSpade => Some(Rank::One),
            Self::TwoClub | Self::TwoDiamond | Self::TwoHeart | Self::TwoSpade => Some(Rank::Two),
            Self::ThreeClub | Self::ThreeDiamond | Self::ThreeHeart | Self::ThreeSpade => {
                Some(Rank::Three)
            }
            Self::FourClub | Self::FourDiamond | Self::FourHeart | Self::FourSpade => {
                Some(Rank::Four)
            }
            Self::FiveClub | Self::FiveDiamond | Self::FiveHeart | Self::FiveSpade => {
                Some(Rank::Five)
            }
            Self::SixClub | Self::SixDiamond | Self::SixHeart | Self::SixSpade => Some(Rank::Six),
            Self::SevenClub | Self::SevenDiamond | Self::SevenHeart | Self::SevenSpade => {
                Some(Rank::Seven)
            }
            Self::EightClub | Self::EightDiamond | Self::EightHeart | Self::EightSpade => {
                Some(Rank::Eight)
            }
            Self::NineClub | Self::NineDiamond | Self::NineHeart | Self::NineSpade => {
                Some(Rank::Nine)
            }
            Self::TenClub | Self::TenDiamond | Self::TenHeart | Self::TenSpade => Some(Rank::Ten),
            Self::JackClub | Self::JackDiamond | Self::JackHeart | Self::JackSpade => {
                Some(Rank::Jack)
            }
            Self::QueenClub | Self::QueenDiamond | Self::QueenHeart | Self::QueenSpade => {
                Some(Rank::Queen)
            }
            Self::KingClub | Self::KingDiamond | Self::KingHeart | Self::KingSpade => {
                Some(Rank::King)
            }
        }
    }
    pub fn get_suit(&self) -> Option<Suit> {
        match self {
            Self::Joker => None,
            Self::OneClub
            | Self::TwoClub
            | Self::ThreeClub
            | Self::FourClub
            | Self::FiveClub
            | Self::SixClub
            | Self::SevenClub
            | Self::EightClub
            | Self::NineClub
            | Self::TenClub
            | Self::JackClub
            | Self::QueenClub
            | Self::KingClub => Some(Suit::Club),
            Self::OneDiamond
            | Self::TwoDiamond
            | Self::ThreeDiamond
            | Self::FourDiamond
            | Self::FiveDiamond
            | Self::SixDiamond
            | Self::SevenDiamond
            | Self::EightDiamond
            | Self::NineDiamond
            | Self::TenDiamond
            | Self::JackDiamond
            | Self::QueenDiamond
            | Self::KingDiamond => Some(Suit::Diamond),
            Self::OneHeart
            | Self::TwoHeart
            | Self::ThreeHeart
            | Self::FourHeart
            | Self::FiveHeart
            | Self::SixHeart
            | Self::SevenHeart
            | Self::EightHeart
            | Self::NineHeart
            | Self::TenHeart
            | Self::JackHeart
            | Self::QueenHeart
            | Self::KingHeart => Some(Suit::Heart),
            Self::OneSpade
            | Self::TwoSpade
            | Self::ThreeSpade
            | Self::FourSpade
            | Self::FiveSpade
            | Self::SixSpade
            | Self::SevenSpade
            | Self::EightSpade
            | Self::NineSpade
            | Self::TenSpade
            | Self::JackSpade
            | Self::QueenSpade
            | Self::KingSpade => Some(Suit::Spade),
        }
    }
}

#[derive(Debug)]
pub struct Card {
    card: CardVariant,
    r#type: Type,
    is_wildcard: bool,
}

impl From<CardVariant> for Card {
    fn from(value: CardVariant) -> Self {
        let (r#type, is_wildcard): (Type, bool) = match value {
            CardVariant::Joker => (Type::Joker, true),
            _ => (
                Type::Normal {
                    rank: value.get_rank().unwrap(),
                    suit: value.get_suit().unwrap(),
                },
                false,
            ),
        };
        Self {
            card: value,
            r#type,
            is_wildcard,
        }
    }
}

#[derive(Debug)]
pub enum Type {
    Normal { rank: Rank, suit: Suit },
    Joker,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Sequence, EnumIter)]
pub enum Rank {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Sequence, EnumIter)]
pub enum Suit {
    Club = 0,
    Diamond = 1,
    Heart = 2,
    Spade = 3,
}

#[derive(Debug)]
pub struct CardCollection(Option<VecDeque<Card>>);

impl CardCollection {
    pub fn new_standard_deck_with_jokers() -> Self {
        Self(Some(
            CardVariant::iter()
                .map(|card_variant| card_variant.into())
                .collect::<VecDeque<Card>>(),
        ))
    }
    pub fn new_double_deck_with_jokers() -> Self {
        Self(Some(
            CardVariant::iter()
                .chain(CardVariant::iter())
                .map(|card_variant| card_variant.into())
                .collect::<VecDeque<Card>>(),
        ))
    }
    pub fn new(cards: Vec<Card>) -> CardCollection {
        Self(Some(cards.into()))
    }
    pub fn output(&self) {
        for card in self.0.as_ref().unwrap().iter() {
            println!("{:#?}", card.card);
        }
    }
    /* pub fn take_card_off_top(&mut self) -> Option<Card> {
        self.0?.pop_back()
    } */

    /* pub fn take_cards_off_top(&mut self, n: u8) -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();
        for _ in 0..n {
            if let
        }
        (0..n).map(|_| ).
        self.cards.pop_back()
    } */

    fn chunk_shuffle(cards: VecDeque<Card>) -> VecDeque<Card> {
        //main deck dumps a chunk of cards on either end of the deck. There will be a 50% chance of it switching to the other end after every iteration.
        let mut cards = cards.into_iter();
        let mut leaning_left: bool = false;
        let mut final_deck: VecDeque<Card> = VecDeque::new();
        'outer: loop {
            let random: u8 = generate_random_number::<u8>(4, 8);
            for _ in 0..random {
                if let Some(card) = cards.next() {
                    if leaning_left {
                        final_deck.push_front(card);
                    } else {
                        final_deck.push_back(card);
                    }
                } else {
                    break 'outer;
                }
            }
            leaning_left = !leaning_left;
        }
        final_deck
    }

    fn zipper_shuffle(mut cards: VecDeque<Card>) -> VecDeque<Card> {
        let b: VecDeque<Card> = cards.split_off(cards.len() / 2);
        let mut a_iter = cards.into_iter();
        let mut b_iter = b.into_iter();
        let mut leaning_left: bool = false;
        let mut final_deck: VecDeque<Card> = VecDeque::new();
        let mut left_empty: bool = false;
        let mut right_empty: bool = false;
        'outer: loop {
            if right_empty && left_empty {
                break;
            }
            let random: u8 = generate_random_number::<u8>(1, 3);
            let current = if leaning_left {
                &mut a_iter
            } else {
                &mut b_iter
            };
            for _ in 0..random {
                if let Some(card) = current.next() {
                    final_deck.push_back(card);
                } else {
                    if leaning_left {
                        left_empty = true;
                    } else {
                        right_empty = true;
                    }
                    leaning_left = !leaning_left;
                    continue 'outer;
                }
            }
            leaning_left = !leaning_left;
        }
        final_deck
    }

    pub fn deal_cards(
        &mut self,
        count_for_each: u8,
        players: &mut HashMap<u8, Player>,
        order: &mut PlayerQueue,
    ) {
        for _ in 0..count_for_each {
            for player_uid in order.queue.iter() {
                if let Some(card) = self.0.as_mut().unwrap().pop_front() {
                    players.get_mut(player_uid).unwrap().add_card_to_hand(card);
                } else {
                    todo!("Handle case where there are no cards available to deal.");
                }
            }
        }
    }

    pub fn take_card_off_top(&mut self) -> Option<Card> {
        self.0.as_mut().unwrap().pop_front()
    }

    pub fn shuffle(&mut self) {
        let mut cards = self.0.take().unwrap();
        for _ in 0..10 {
            cards = Self::chunk_shuffle(cards);
            cards = Self::zipper_shuffle(cards);
        }

        self.0 = Some(cards);
    }
}

/* pub struct Hand {
    cards: VecDeque<Card>,
} */

#[derive(Debug)]
pub struct Player {
    uid: u8,
    hand: VecDeque<Card>,
}

impl Player {
    pub fn new(uid: u8) -> Self {
        Self {
            uid,
            hand: Default::default(),
        }
    }
    pub fn add_card_to_hand(&mut self, card: Card) {
        self.hand.push_back(card);
    }
    pub fn output(&self) {
        for (i, card) in self.hand.iter().enumerate() {
            println!("{:2}: {:?}", i, card.card);
        }
    }
}

#[derive(Debug)]
pub struct PlayerQueue {
    queue: VecDeque<u8>,
}

impl PlayerQueue {
    pub fn new(player_count: u8, starting_player: u8) -> Self {
        let mut t = Self {
            queue: (0..player_count).collect(),
        };
        t.cycle_to_player(starting_player);
        t
    }

    pub fn cycle(&mut self) {
        let t = self.queue.pop_front().unwrap();
        self.queue.push_back(t);
    }

    fn cycle_to_player(&mut self, player: u8) {
        loop {
            if self.queue[0] != player {
                self.cycle();
            } else {
                break;
            }
        }
    }
}

pub fn play_313(player_count: u8) {
    fn prompt_boolean() -> bool {
        let mut input = String::new();
        loop {
            println!("Enter \"true\" or \"false\"");
            if let Err(err) = io::stdin().read_line(&mut input) {
                eprintln!("Please enter correct input: {err}");
            } else {
                match input.trim().to_lowercase().as_str() {
                    "true" => return true,
                    "false" => return false,
                    _ => {
                        eprintln!("Invalid input");
                        input.clear();
                    }
                }
            }
        }
    }

    fn prompt_card_discard(total: u8) -> u8 {
        let mut input = String::new();
        loop {
            println!("Please select which card you would like to discard by entering number between 0 and {}", total - 1);
            if let Err(err) = io::stdin().read_line(&mut input) {
                eprintln!("Invalid input: {err}");
            } else {
                match input.trim().parse::<u8>() {
                    Ok(confirmation) => {
                        if confirmation > (total - 1) {
                            eprintln!("Number must be between 0 and {}.", total - 1);
                            input.clear();
                        } else {
                            return confirmation;
                        }
                    }
                    Err(err) => {
                        eprintln!("Invalid input: {err}");
                        input.clear();
                    }
                }
            }
        }
    }
    if player_count < 2 || player_count > 4 {
        println!("Game requires between 2 - 4 players.");
        return;
    }
    let mut players: HashMap<u8, Player> = (0..player_count)
        .into_iter()
        .map(|i| (i, Player::new(i)))
        .collect::<HashMap<u8, Player>>();

    let mut player_queue: PlayerQueue = PlayerQueue::new(player_count, 0);

    let mut card_collection: CardCollection = match player_count {
        2 => CardCollection::new_standard_deck_with_jokers(),
        3 | 4 => CardCollection::new_double_deck_with_jokers(),
        _ => todo!("More than 4 players is currently unsupported."),
    };

    card_collection.shuffle();
    card_collection.deal_cards(4, &mut players, &mut player_queue);
    let mut pile: VecDeque<Card> = VecDeque::new();
    pile.push_front(card_collection.take_card_off_top().unwrap());
    loop {
        let player = players.get_mut(&player_queue.queue[0]).unwrap();

        //Output players hand at the start of their turn
        println!("Player {}'s hand:", player.uid);
        player.output();

        //Either pickup from the pile or from the deck
        println!(
            "Do you want to pickup card {:?} from pile?",
            pile.iter().last().unwrap().card
        );
        if prompt_boolean() {
            player.hand.push_back(pile.pop_front().unwrap());
        } else {
            player
                .hand
                .push_back(card_collection.take_card_off_top().unwrap()); //TODO: This unwrap needs to be handled.
        }

        //Output their hand after they pickup a card.
        println!("Player {}'s new hand state:", player.uid);
        player.output();

        //Discard card from their hand and move to the pile
        let card_to_discard = prompt_card_discard(player.hand.len() as u8);
        pile.push_front(player.hand.remove(card_to_discard as usize).unwrap());

        //Cycle to the next player
        player_queue.cycle();
    }
}

fn main() {
    play_313(2);
}
