#[derive(Debug)]
pub struct Grid {
    dimensions: usize,
    radius: f32,
    pockets: Vec<Vec<Vec<usize>>>,
}
// TODO: rozpisać na kartce jak to właściwie ma działać, jak konwertowac, jakie są granice każdej komórki..
// czy będzie trzeba zupełnie przepisać wszystko by ten grid obsłużyć?
impl Grid {
    pub fn new(radius: f32) -> Self {
        let dimensions = (2.0 / radius) as usize + 1;

        Self {
            dimensions,
            radius,
            pockets: vec![vec![vec![]; dimensions]; dimensions],
        }
    }

    pub fn push(&mut self, idx: usize, x: f32, y: f32) {
        let (i, j) = self.get_ij(x, y);

        let x = self.pockets.get_mut(i).unwrap();
        let y = x.get_mut(j).unwrap();
        y.push(idx);
    }

    pub fn log_dim(&self) {
        println!("{}", self.dimensions);
    }

    pub fn log(&self) {
        println!("{:?}", self);
    }

    pub fn get(&self, x: usize, y: usize) -> &Vec<usize> {
        (self.pockets.get(x)).unwrap().get(y).unwrap()
    }

    pub fn get_ij(&self, x: f32, y: f32) -> (usize, usize) {
        (
            ((x + 1.0) / self.radius) as usize,
            ((y + 1.0) / self.radius) as usize,
        )
    }
}
