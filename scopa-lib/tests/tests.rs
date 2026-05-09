use scopa_lib::card::*;
use scopa_lib::player::Player;

fn player_with_cards() -> Player {
    let mut p = Player::new("test");
    p.take_cards(vec![
        Card {
            suite: Suite::Coins,
            value: CardValue::Seven,
        },
        Card {
            suite: Suite::Coins,
            value: CardValue::Six,
        },
        Card {
            suite: Suite::Coins,
            value: CardValue::One,
        },
        Card {
            suite: Suite::Coins,
            value: CardValue::Re,
        },
        Card {
            suite: Suite::Swords,
            value: CardValue::One,
        },
        Card {
            suite: Suite::Swords,
            value: CardValue::Cavallo,
        },
        Card {
            suite: Suite::Cups,
            value: CardValue::One,
        },
        Card {
            suite: Suite::Cups,
            value: CardValue::Six,
        },
        Card {
            suite: Suite::Clubs,
            value: CardValue::Two,
        },
        Card {
            suite: Suite::Clubs,
            value: CardValue::Fante,
        },
        Card {
            suite: Suite::Clubs,
            value: CardValue::Re,
        },
    ]);
    p
}

#[test]
fn take_cards() {
    let p = player_with_cards();
    assert_eq!(p.taken.count(), 11);
    assert_eq!(p.taken.coins.len(), 4);
    assert_eq!(p.taken.swords.len(), 2);
    assert_eq!(p.taken.cups.len(), 2);
    assert_eq!(p.taken.clubs.len(), 3);
}

#[test]
fn get_primes() {
    let p = player_with_cards();
    assert_eq!(p.taken.primes(), 67);
}

#[test]
fn check_results() {
    let p = player_with_cards();
    let r = p.results();
    assert_eq!(r.scopas, 0);
    assert_eq!(r.takes, 11);
    assert_eq!(r.primes, 67);
    assert_eq!(r.count_of_coins, 4);
    assert!(r.seven_of_coins);
}

#[test]
fn compare_cards() {
    let c1 = Card {
        suite: Suite::Coins,
        value: CardValue::Seven,
    };
    let c2 = Card {
        suite: Suite::Coins,
        value: CardValue::Seven,
    };
    let c3 = Card {
        suite: Suite::Swords,
        value: CardValue::Seven,
    };
    let c4 = Card {
        suite: Suite::Coins,
        value: CardValue::Six,
    };
    let c5 = Card {
        suite: Suite::Swords,
        value: CardValue::Six,
    };
    assert_eq!(c1, c2);
    assert_ne!(c1, c3);
    assert_ne!(c1, c4);
    assert_ne!(c1, c5);
}

#[test]
fn table_contains_same_value() {
    let mut table = Table::default();
    use CardValue::*;
    use Suite::*;
    table.put_card(Card {
        suite: Coins,
        value: Two,
    });
    table.put_card(Card {
        suite: Cups,
        value: Five,
    });
    table.put_card(Card {
        suite: Swords,
        value: Two,
    });
    table.put_card(Card {
        suite: Coins,
        value: Four,
    });

    let card = Card {
        suite: Coins,
        value: Five,
    };

    assert_eq!(
        table.contains_same_value(&card),
        Some(&Card {
            suite: Cups,
            value: Five
        })
    );

    let card = Card {
        suite: Coins,
        value: Re,
    };

    assert_eq!(table.contains_same_value(&card), None);
}
