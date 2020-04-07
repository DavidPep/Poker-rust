use std::cmp;
fn main() {
    let perm:[u32;10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; 
    let winner:[String;5] = deal(&perm);
    println!("{:?}", winner);
    
}

fn formatting (array: [(i32, String); 5]) -> [String;5]
{
    let mut out:[String;5] = ["".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string() ]; 
    let mut count = 0;
    for card in array.iter()
    {
        if count < 5
        {
            let elem = card.0.to_string() + &card.1.to_string();
            out[count] = elem;
        }
        count += 1;
    }
    return out;
}

fn deal  <'a> (hand: &'a [u32; 10]) ->  [String;5]
{
    let (mut odd, mut even) = split(&hand);
    odd.sort();
    even.sort();
    let winner = compare(&create_deck(odd), &create_deck(even));
    return formatting(winner.clone());

}

fn create_deck <'a> (array:  Vec<&'a u32>) -> [(i32, String);5]
{
    //let mut a = array.sort();
    let mut deck:[(i32, String);5] = [(0," ".to_string()),(0," ".to_string()),(0," ".to_string()),(0," ".to_string()),(0," ".to_string())];
    for i in 0..5 {
        deck[i] = create_card(*array[i]);
    }
    return deck;
}
/// translates a value between 1 and 52 to a Card. Used internally.
fn create_card(value: u32) -> (i32, String) 
{

    let suit = match value {
        0..=13 => "C",
        14..=26 => "D",
        27..=39 => "H",
        40..=52 => "S",
        _ => panic!("Unexpected suit conversion number")
    };

    let rank = match value%13 {
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 6,
        7 => 7,
        8 => 8,
        9 => 9,
        10 => 10,
        11 => 11,
        12 => 12,
        0 => 13,
        _ => panic!("Unexpected value conversion number")
    };

    return (rank, suit.to_string());
}



fn split <'a> (arr: &'a [u32; 10]) -> (Vec<&'a u32>, Vec<&'a u32>)
{
    let mut hand1 = Vec::new();
    let mut hand2 = Vec::new();
    let mut n = 0;
    for elem in arr.iter(){
        if n%2 == 0 {
            hand1.push(elem); 
        }
        else{
            hand2.push(elem); 
        }
        n += 1;
    }
    let x = (hand1, hand2);
    return x;
} 

fn get_score <'a>(hand: &'a [(i32, String); 5])-> i32
{
    let score = match identify_hand(hand).as_str() {
        "Royal Flush" => 10,
        "Straight Flush" => 9,
        "Four of a Kind" => 8,
        "Full House" => 7,
        "Flush" => 6,
        "Straight" => 5,
        "Three of a Kind" => 4,
        "Two Pair" => 3,
        "Pair" => 2,
        "High Card" => 1,
        _ => panic!("no value")
    };
    return score;

}
fn tie_breaker_suit <'a> (hand: &'a [(i32, String); 5]) -> i32
{
    let score = match hand[0].1.as_str(){
        "C" => 1,
        "D" => 2,
        "H" => 3,
        "S" => 4,
        _ => panic!("no value")
    };
    return score;
}
fn compare <'a> (hand1: &'a [(i32, String); 5], hand2: &'a [(i32, String); 5])->  [(i32, String); 5]
{
    if get_score(hand1) > get_score(hand2)
    {
        return hand1.clone();
    }
    else if get_score(hand1) < get_score(hand2)
    {
        return hand2.clone();
    }
    else
    {
        return tie_breaker(&hand1.clone(), &hand2.clone());
    }
    
}
fn tie_breaker <'a> (hand1: &'a [(i32, String); 5], hand2: &'a [(i32, String); 5]) ->  [(i32, String); 5]
{
    if get_score(hand1) == 10
    {
        if tie_breaker_suit(&hand1) > tie_breaker_suit(&hand2)
        {
            return hand1.clone();
        }
        else
        {
            return hand2.clone();
        }
    }

    else if get_score(hand1) == 9
    {

        if hand1[4].0 > hand2[4].0
        {
            return hand1.clone();
        }
        else if hand1[4].0 < hand2[4].0
        {
            return hand2.clone();
        }
        else if tie_breaker_suit(&hand1) > tie_breaker_suit(&hand2)
        {
            return hand1.clone();
        }
        else
        {
            return hand2.clone();
        }
    }

    else if get_score(hand1) == 8
    {
        if  majority(hand1) > majority(hand2)
        {
            return hand1.clone();
        }
        else if  majority(hand1) < majority(hand2)
        {
            return hand2.clone();
        }
        else if tie_breaker_suit(&hand1) > tie_breaker_suit(&hand2)
        {
            return hand1.clone();
        }
        else
        {
            return hand2.clone();
        }

    }
    else if get_score(hand1) == 7
    {

        if  majority(hand1) > majority(hand2)
        {
            return hand1.clone();
        }
        else if  majority(hand1) < majority(hand2)
        {
            return hand2.clone();
        }
        else if tie_breaker_suit(&hand1) > tie_breaker_suit(&hand2)
        {
            return hand1.clone();
        }
        else
        {
            return hand2.clone();
        }

    }
    else if get_score(hand1) == 6
    {
        if hand1[4].0 > hand2[4].0 || hand1[0].0 == 1
        {
            return hand1.clone();
        }
        else if hand1[4].0 < hand2[4].0 || hand2[0].0 == 1
        {
            return hand2.clone();
        }
        else if tie_breaker_suit(&hand1) > tie_breaker_suit(&hand2)
        {
            return hand1.clone();
        }
        else
        {
            return hand2.clone();
        }

    }
    else if get_score(hand1) == 5
    {
        if hand1[4].0 > hand2[4].0 || hand1[0].0 < hand2[0].0
        {
            return hand1.clone();
        }
        else if hand1[4].0 < hand2[4].0 || hand1[0].0 > hand2[0].0
        {
            return hand2.clone();
        }
        else if tie_breaker_suit(&hand1) > tie_breaker_suit(&hand2)
        {
            return hand1.clone();
        }
        else
        {
            return hand2.clone();
        }

    }
    else if get_score(hand1) == 4
    {
        if  majority(hand1) > majority(hand2)
        {
            return hand1.clone();
        }
        else if  majority(hand1) < majority(hand2)
        {
            return hand2.clone();
        }
        else if tie_breaker_suit(&hand1) > tie_breaker_suit(&hand2)
        {
            return hand1.clone();
        }
        else
        {
            return hand2.clone();
        }

    }
    else if get_score(hand1) == 3
    {
        if  majority(hand1) > majority(hand2)
        {
            return hand1.clone();
        }
        else if  majority(hand1) < majority(hand2)
        {
            return hand2.clone();
        }
        else if tie_breaker_suit(&hand1) > tie_breaker_suit(&hand2)
        {
            return hand1.clone();
        }
        else
        {
            return hand2.clone();
        }

    }
    else if get_score(hand1) == 2
    {
        if  majority(hand1) > majority(hand2)
        {
            return hand1.clone();
        }
        else if  majority(hand1) < majority(hand2)
        {
            return hand2.clone();
        }
        else if tie_breaker_suit(&hand1) > tie_breaker_suit(&hand2)
        {
            return hand1.clone();
        }
        else
        {
            return hand2.clone();
        }

    }
    else
    {
        panic!("bad input");
    }
}

fn identify_hand <'a> (hand: &'a [(i32, String); 5]) -> String
{
    // is royal flush
    if is_royal_flush(hand){
        return "Royal Flush".to_string();
    }

    // is straight flush
    else if is_straight_flush(hand){
        return "Straight Flush".to_string();
    }

    //four of a kind
    else if is_four(hand){
        return "Four of a Kind".to_string();
    }

    //full house
    else if is_full_house(hand){
        return "Full House".to_string();
    }
    //flush
    else if is_flush(hand){
        return "Flush".to_string();
    }

    //straight
    else if is_straight (hand){
        return "Straight".to_string();
    }

    //three of a kind
    else if is_three(hand){
        return "Three of a Kind".to_string();
    }

    //two pair
    else if is_two(hand){
        return "Two Pair".to_string();
    }

    //pair
    else if is_pair(hand){
        return "Pair".to_string();
    }

    //high card
    else
    {
        return "High Card".to_string();
    }

}

// royal flush
fn is_royal_flush <'a> (list: &'a [(i32, String); 5]) -> bool
{
    let (mut ccount, mut dcount, mut hcount, mut scount)  = (0,0,0,0);
    let royal: bool =  list[0].0 == 1 && list[1].0 == 10 && list[2].0 == 11 && list[3].0 == 12 && list[4].0 == 13;
    for card in list.iter()
    {
        if card.1 == "C"
        {
            ccount += 1;
        }
        if card.1 == "D"
        {
            dcount += 1;
        }
        if card.1 == "H"
        {
            hcount += 1;
        }
        if card.1 == "S"
        {
            scount += 1;
        }
    }
    if dcount == 5 || ccount == 5 || hcount == 5 || scount == 5 
    {
        return royal;
    }
    else {
        return false;
    }
}

// straight flush
fn is_straight_flush <'a> (list: &'a [(i32, String); 5]) -> bool
{
    let (mut ccount, mut dcount, mut hcount, mut scount)  = (0,0,0,0);
    let consecutive: bool =  list[1].0 == list[0].0 + 1 && list[2].0 == list[1].0 + 1 && list[3].0 == list[2].0 + 1 && list[4].0 == list[3].0 + 1;
    for card in list.iter()
    {
        if card.1 == "C"
        {
            ccount += 1;
        }
        if card.1 == "D"
        {
            dcount += 1;
        }
        if card.1 == "H"
        {
            hcount += 1;
        }
        if card.1 == "S"
        {
            scount += 1;
        }
    }
    if dcount == 5 || ccount == 5 || hcount == 5 || scount == 5 
    {
        return consecutive;
    }
    else {
        return false;
    }

}
//four of a kind
fn is_four <'a> (list: &'a [(i32, String); 5]) -> bool
{
    let (mut ccount, mut dcount, mut hcount, mut scount)  = (0,0,0,0);
    let (mut a, mut b, mut c, mut d, mut e)  = (0,0,0,0,0);
    for card in list.iter()
    {
        if card.0 == list[0].0
        {
            a += 1;
        }
        if card.0 == list[1].0
        {
            b += 1;
        }
        if card.0 == list[2].0
        {
            c += 1;
        }
        if card.0 == list[3].0
        {
            d += 1;
        }
        if card.0 == list[4].0
        {
            e += 1;
        }
        
    }
    if a == 4 || b == 4 || c == 4 || d == 4 || e == 4
    {
        return true;
    }
    else
    {

    for card in list.iter()
    {
        if card.1 == "C"
        {
            ccount += 1;
        }
        if card.1 == "D"
        {
            dcount += 1;
        }
        if card.1 == "H"
        {
            hcount += 1;
        }
        if card.1 == "S"
        {
            scount += 1;
        }
    }
    if dcount == 4 || ccount == 4 || hcount == 4 || scount == 4 
    {
        return true;
    }
    else
    {
        return false;
    }
    }
}

//pair
fn is_pair <'a> (list: &'a [(i32, String); 5]) -> bool
{
    let (mut ccount, mut dcount, mut hcount, mut scount)  = (0,0,0,0);
    let (mut a, mut b, mut c, mut d, mut e)  = (0,0,0,0,0);
    for card in list.iter()
    {
        if card.0 == list[0].0
        {
            a += 1;
        }
        if card.0 == list[1].0
        {
            b += 1;
        }
        if card.0 == list[2].0
        {
            c += 1;
        }
        if card.0 == list[3].0
        {
            d += 1;
        }
        if card.0 == list[4].0
        {
            e += 1;
        }
        
    }
    if a == 2 || b == 2 || c == 2 || d == 2 || e == 2
    {
        return true;
    }
    else
    {

    for card in list.iter()
    {
        if card.1 == "C"
        {
            ccount += 1;
        }
        if card.1 == "D"
        {
            dcount += 1;
        }
        if card.1 == "H"
        {
            hcount += 1;
        }
        if card.1 == "S"
        {
            scount += 1;
        }
    }
    if dcount == 2 || ccount == 2 || hcount == 2 || scount == 2 
    {
        return true;
    }
    else
    {
        return false;
    }
    }
}

//three of a kind
fn is_three <'a> (list: &'a[(i32, String); 5]) -> bool
{
    let (mut ccount, mut dcount, mut hcount, mut scount)  = (0,0,0,0);
    let (mut a, mut b, mut c, mut d, mut e)  = (0,0,0,0,0);
    for card in list.iter()
    {
        if card.0 == list[0].0
        {
            a += 1;
        }
        if card.0 == list[1].0
        {
            b += 1;
        }
        if card.0 == list[2].0
        {
            c += 1;
        }
        if card.0 == list[3].0
        {
            d += 1;
        }
        if card.0 == list[4].0
        {
            e += 1;
        }
        
    }
    if a == 3 || b == 3 || c == 3 || d == 3 || e == 3
    {
        return true;
    }
    else
    {

    for card in list.iter()
    {
        if card.1 == "C"
        {
            ccount += 1;
        }
        if card.1 == "D"
        {
            dcount += 1;
        }
        if card.1 == "H"
        {
            hcount += 1;
        }
        if card.1 == "S"
        {
            scount += 1;
        }
    }
    if dcount == 3 || ccount == 3 || hcount == 3 || scount == 3
    {
        return true;
    }
    else
    {
        return false;
    }
    }
}

//straight
fn is_straight <'a> (list: &'a [(i32, String); 5]) -> bool
{
    let consecutive: bool =  list[1].0 == list[0].0 + 1 && list[2].0 == list[1].0 + 1 && list[3].0 == list[2].0 + 1 && list[4].0 == list[3].0 + 1;
    return consecutive;

}

//full house
fn is_full_house <'a> (list: &'a [(i32, String); 5]) -> bool
{
    if is_three(list) && is_pair(list)
    {
        return true;
    }
    else
    {
        return false;
    }
}


//flush
fn is_flush <'a> (list: &'a [(i32, String); 5]) -> bool
{
    let (mut ccount, mut dcount, mut hcount, mut scount)  = (0,0,0,0);
    {
        for card in list.iter()
    {
        if card.1 == "C"
        {
            ccount += 1;
        }
        if card.1 == "D"
        {
            dcount += 1;
        }
        if card.1 == "H"
        {
            hcount += 1;
        }
        if card.1 == "S"
        {
            scount += 1;
        }
    }
    }
    if dcount == 5 || ccount == 5 || hcount == 5 || scount == 5 
    {
        return true;
    }
    else {
        return false;
    }

}

//two pair
fn is_two <'a> (list: &'a [(i32, String); 5]) -> bool
{
    let (mut a, mut b, mut c, mut d, mut e, mut t)  = (0,0,0,0,0,0);
    for card in list.iter()
    {
        if card.0 == list[0].0
        {
            a += 1;
        }
        if card.0 == list[1].0
        {
            b += 1;
        }
        if card.0 == list[2].0
        {
            c += 1;
        }
        if card.0 == list[3].0
        {
            d += 1;
        }
        if card.0 == list[4].0
        {
            e += 1;
        }
        
    }
    let arr = [a, b, c, d, e];
    for two in arr.iter()
    {
        if two == &2
        {
            t += 1;
        }
    }
    if t == 2
    {
        return true;
    }
    else
    {
        return false;
    }
}

fn majority <'a> (list: &'a [(i32, String); 5]) -> i32
{
    let (mut a, mut b, mut c, mut d, mut e)  = (0,0,0,0,0);
    for card in list.iter()
    {
        if card.0 == list[0].0
        {
            a += 1;
        }
        if card.0 == list[1].0
        {
            b += 1;
        }
        if card.0 == list[2].0
        {
            c += 1;
        }
        if card.0 == list[3].0
        {
            d += 1;
        }
        if card.0 == list[4].0
        {
            e += 1;
        }
        
    }
    let one = cmp::max(a, b);
    let two = cmp::max(c, d);
    let three = cmp::max(one, two);
    let max = cmp::max(three, e);
    if max == a
    {
        return list[0].0;
    }
    if max == b
    {
        return list[1].0;
    }
    if max == c
    {
        return list[2].0;
    }
    if max == d
    {
        return list[3].0;
    }
    if max == e
    {
        return list[4].0;
    }
    return 0;
    


}