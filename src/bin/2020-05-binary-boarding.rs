fn main() {
    let partitions = vec![
        BinaryPartition::Lower,
        BinaryPartition::Upper,
        BinaryPartition::Lower,
        BinaryPartition::Upper,
        BinaryPartition::Upper,
        BinaryPartition::Lower,
        BinaryPartition::Lower,
    ];
    println!("{}", partition_location(&partitions));
}

#[derive(Debug)]
enum BinaryPartition {
    Upper,
    Lower,
}

fn partition_location(partitions: &Vec<BinaryPartition>) -> u32 {
    let mut min = 0;
    let mut max = 2_u32.pow(partitions.len() as u32) - 1;

    for partition in partitions {
        let midpoint = min + (max - min + 1) / 2;
        match partition {
            BinaryPartition::Upper => {
                min = midpoint;
            }
            BinaryPartition::Lower => {
                max = midpoint - 1;
            }
        }
    }

    assert_eq!(max, min);

    max
}
