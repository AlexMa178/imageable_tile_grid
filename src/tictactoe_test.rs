use crate::{ CharTile, Tile, TileGrid };

#[derive(Clone, Copy)]
enum TicTacToeTile {
    X, O, Empty
}
impl Tile for TicTacToeTile {
    const SIZE: usize = 3;
    fn atlas_pos(&self) -> [usize; 2] {
        match self {
            TicTacToeTile::X => [ 0, 0 ],
            TicTacToeTile::O => [ 1, 0 ],
            TicTacToeTile::Empty => [ 2, 0 ],
        }
    }
}
impl CharTile for TicTacToeTile {
    fn from_char(c: char) -> Self {
        match c {
            'X' => Self::X,
            'O' => Self::O,
            ' ' => Self::Empty,
            _ => panic!("invalid char"),
        }
    }
}

const TIC_TAC_TOE_ATLAS: [u8; 3*3*4 * 3] = [
    0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    255,255,255,255,    255,255,255,255,    255,255,255,255,
    255,255,255,255,    0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    255,255,255,255,    255,255,255,255,
    0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    255,255,255,255,    255,255,255,255,    255,255,255,255,
];

const TIC_TAC_TOE_COMPOSED: [u8; 3*3*4 * 3 * 3] = [
    0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    255,255,255,255,    255,255,255,255,    255,255,255,255,    0,0,0,255,          255,255,255,255,
    255,255,255,255,    0,0,0,255,          255,255,255,255,    255,255,255,255,    255,255,255,255,    255,255,255,255,    0,0,0,255,          255,255,255,255,    0,0,0,255,      
    0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    255,255,255,255,    255,255,255,255,    255,255,255,255,    0,0,0,255,          255,255,255,255,
    0,0,0,255,          255,255,255,255,    0,0,0,255,          0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    255,255,255,255,    255,255,255,255,
    255,255,255,255,    0,0,0,255,          255,255,255,255,    255,255,255,255,    0,0,0,255,          255,255,255,255,    255,255,255,255,    255,255,255,255,    255,255,255,255,
    0,0,0,255,          255,255,255,255,    0,0,0,255,          0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    255,255,255,255,    255,255,255,255,
    0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    255,255,255,255,    0,0,0,255,          255,255,255,255,
    255,255,255,255,    0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    0,0,0,255,          0,0,0,255,          255,255,255,255,    0,0,0,255,      
    0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    0,0,0,255,          255,255,255,255,    255,255,255,255,    0,0,0,255,          255,255,255,255,
];

#[test]
fn compose() {

    let mut grid: TileGrid<TicTacToeTile, 3, 3> = TileGrid::fill(TicTacToeTile::Empty);
    grid.grid[0][0] = TicTacToeTile::X;
    grid.grid[0][2] = TicTacToeTile::O;
    grid.grid[1][0] = TicTacToeTile::X;
    grid.grid[1][1] = TicTacToeTile::X;
    grid.grid[2][0] = TicTacToeTile::X;
    grid.grid[2][1] = TicTacToeTile::O;
    grid.grid[2][2] = TicTacToeTile::O;
    let composed_image = grid.compose_image(&TIC_TAC_TOE_ATLAS, 3);
    assert_eq!(composed_image, TIC_TAC_TOE_COMPOSED.to_vec());

}

#[test]
fn view_and_compose() {

    let mut grid: TileGrid<TicTacToeTile, 3, 3> = TileGrid::fill(TicTacToeTile::Empty);
    grid.view(0, 0).write("X O\r\nXX \r\nXOO");
    let composed_image = grid.compose_image(&TIC_TAC_TOE_ATLAS, 3);
    assert_eq!(composed_image, TIC_TAC_TOE_COMPOSED.to_vec());

}