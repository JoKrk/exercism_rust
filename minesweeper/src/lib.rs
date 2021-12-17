
struct MineSpot {
    row: u8,
    col: u8,
    neighbours: u8
}


pub fn annotate(minefield: &[&str]) -> Vec<String> {

    //git commit change

}

fn create_table(minefield: &[&str]) -> Vec<MineSpot> {

    let mut spots = Vec::new();

    for m in minefield {
        for n in m.chars() {
            
        }
    }

    spots
}