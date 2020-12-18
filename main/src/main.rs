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
}
