#[derive(Debug)]
pub struct Grid {
    dimensions: usize,
    radius: f32,
    pub pockets: Vec<Vec<Vec<usize>>>,
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

    pub fn bogus() -> Self {
        Self {
            dimensions: 0,
            radius: 0.0,
            pockets: vec![vec![vec![]; 0]; 0],
        }
    }

    pub fn push(&mut self, idx: usize, x: f32, y: f32) {
        let (i, j) = self.get_ij(x, y);

        let column = self.pockets.get_mut(i).unwrap();
        let row = column.get_mut(j).unwrap();

        row.push(idx);
    }

    pub fn clear(&mut self) {
        for v_v in &mut self.pockets {
            for v in v_v {
                v.clear();
            }
        }
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
