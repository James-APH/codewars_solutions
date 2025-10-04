fn better_than_average(class_points: &[u16], your_points: u16) -> bool {
    let class_average =
        (class_points.iter().sum::<u16>() + your_points) / class_points.len() as u16 + 1;
    your_points > class_average
}
