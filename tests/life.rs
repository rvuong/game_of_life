use life::{GameOfLife, Matrix};

#[test]
fn test_matrix_init() {
    let matrix = Matrix::new(15, 15);

    assert_eq!(matrix.data.len(), 15);
    assert_eq!(matrix.data.first().expect("Reason").len(), 15);
}

#[test]
fn test_game_of_life_init() {
    let game_of_life = GameOfLife::new(15, 15);

    assert_eq!(game_of_life.steps.len(), 1);
}

#[test]
fn test_game_of_life_next() {
    let mut game_of_life = GameOfLife::new(3, 3);
    let next = game_of_life.next();

    assert!(next.is_some());
    assert_eq!(game_of_life.steps.len(), 2);
}
