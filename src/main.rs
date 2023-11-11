use rstar::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Data {
    coordinates: [i32; 18],
}

impl Point for Data {
    type Scalar = i32;
    const DIMENSIONS: usize = 18;

    fn nth(&self, index: usize) -> Self::Scalar {
        self.coordinates[index]
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        &mut self.coordinates[index]
    }

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
        Data {
            coordinates: [
                generator(0),
                generator(1),
                generator(2),
                generator(3),
                generator(4),
                generator(5),
                generator(6),
                generator(7),
                generator(8),
                generator(9),
                generator(10),
                generator(11),
                generator(12),
                generator(13),
                generator(14),
                generator(15),
                generator(16),
                generator(17),
            ],
        }
    }
}

fn main() {
    let mut tree = rstar::RTree::new();
    tree.insert(Data {
        coordinates: [
            30, 30, 62, 30, 31, 30, 20, 12, 30, 80, 40, 132, 28, 78, 140, 139, 195, 67,
        ],
    });
    tree.insert(Data {
        coordinates: [
            40, 9, 9, 144, 136, 149, 148, 140, 198, 36, 50, 167, 179, 234, 38, 2, 103, 38,
        ],
    });
    tree.insert(Data {
        coordinates: [
            38, 38, 38, 6, 2, 3, 155, 61, 61, 195, 155, 15, 70, 134, 158, 126, 94, 63,
        ],
    });
    tree.insert(Data {
        coordinates: [
            154, 154, 138, 179, 121, 75, 143, 31, 7, 67, 11, 3, 113, 113, 65, 65, 73, 65,
        ],
    });
    tree.insert(Data {
        coordinates: [
            221, 215, 209, 115, 210, 198, 224, 236, 111, 6, 7, 7, 85, 92, 203, 197, 36, 44,
        ],
    });
    tree.insert(Data {
        coordinates: [
            176, 240, 176, 176, 112, 241, 240, 240, 192, 44, 44, 12, 206, 204, 142, 4, 44, 2,
        ],
    });
    tree.insert(Data {
        coordinates: [
            212, 218, 202, 70, 70, 23, 23, 23, 1, 230, 250, 55, 30, 23, 60, 60, 92, 16,
        ],
    });
}
