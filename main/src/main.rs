fn main() {
    println!("==== day 1: Helping our accounting Elves ====");
    let ((checksum_1, v1, v2), (checksum_2, v3, v4, v5)) = day1::solve();
    println!("The Elves search for the numbers {v1} and {v2} giving the checksum {checksum_1}", v1 = v1, v2 = v2, checksum_1 = checksum_1);
    println!("The Elves also search for the numbers {v3}, {v4} and {v5} giving the checksum {checksum_2}", v3 = v3, v4 = v4, v5 = v5, checksum_2 = checksum_2);


    println!("\n==== day 2: Helping the shopkeeper at the toboggan rental shop ====");
    let (valid_1, valid_2) = day2::solve();
    println!("According to the old rules, there are {} valid passwords in the database.", valid_1);
    println!("Examining them with the new policy tells us, that there actually are {} valid passwords.", valid_2);

    println!("\n==== day 3: Examining the route to the airport ====");
    let (tree_hits_3_1, tree_hit_score) = day3::solve();
    println!("With the anticipated slope of 3 right, one down we would hit {} trees.", tree_hits_3_1);
    println!("Examining the possible slopes, we calculate our tree hit score of {}", tree_hit_score);

    println!("\n==== day 4: Passport chaos at the airport ====");
    let (valid_passports, strict_valid_passports) = day4::solve();
    println!("Examining the passports around us, we find {} passports with the correct fields.", valid_passports);
    println!("Validating the passports though, we discover only {} passports are valid.", strict_valid_passports);

    println!("\n==== day 5: todo ====");
    let (a, b) = day5::solve();
    println!("\n==== day 6: todo ====");
    let (a, b) = day6::solve();
    println!("\n==== day 7: todo ====");
    let (a, b) = day7::solve();
    println!("\n==== day 8: todo ====");
    let (a, b) = day8::solve();
    println!("\n==== day 9: todo ====");
    let (a, b) = day9::solve();
    println!("\n==== day 10: todo ====");
    let (a, b) = day10::solve();
    println!("\n==== day 11: todo ====");
    let (a, b) = day11::solve();
    println!("\n==== day 12: todo ====");
    let (a, b) = day12::solve();
    println!("\n==== day 13: todo ====");
    let (a, b) = day13::solve();
    println!("\n==== day 14: todo ====");
    let (a, b) = day14::solve();
    println!("\n==== day 15: todo ====");
    let (a, b) = day15::solve();
    println!("\n==== day 16: todo ====");
    let (a, b) = day16::solve();
    println!("\n==== day 17: todo ====");
    let (a, b) = day17::solve();
    println!("\n==== day 18: todo ====");
    let (a, b) = day18::solve();
}
