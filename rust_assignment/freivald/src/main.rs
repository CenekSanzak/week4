// TODO: Import necessary libraries. Check cargo.toml and the documentation of the libraries.
use ark_bls12_381::Fq;
use rand::thread_rng;
use ark_ff::UniformRand;
use ndarray::{Array2};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

struct Freivald {
    x:  Array2<Fq> // Array/Vec of Fq, bonus is done 
}

impl Freivald {
    // DONE TODO: Create constructor for object
    fn new(array_size: usize) -> Self {

        let ref mut rng = thread_rng();
        let mut matrice = Array2::<Fq>::default((10, array_size));
        for i in 0..10{
            for j in 0..array_size{
                matrice[[i, j]] = Fq::rand(rng);
            }
        }
        Freivald {
            x: matrice,
        }
        // Generate random number
        // Populate vector with values r^i for i=0..matrix_size
        // Return freivald value with this vector as its x value
    }

    // DONE TODO: Add proper types to input matrices. Remember matrices should hold Fq values
    fn verify(&self, matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
        assert!(check_matrix_dimensions(matrix_a, matrix_b, supposed_ab));
        self.x.rows().into_iter().all(|r| {
            let br = matrix_b.dot(&r);
            let abr = matrix_a.dot(&br);
            let abr_supposed = supposed_ab.dot(&r);
            abr == abr_supposed
        })
        // TODO: check if a * b * x == c * x. Check algorithm to make sure order of operations are
        // correct
    }

    // utility function to not have to instantiate Freivalds if you just want to make one
    // verification.
    // DONE TODO: Add types for arguments
    fn verify_once(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
        let freivald = Freivald::new(supposed_ab.nrows());
        freivald.verify(matrix_a, matrix_b, supposed_ab)
    }
}
// Done TODO: [Bonus] Modify code to increase your certainty that A * B == C by iterating over the protocol. 
// Note that you need to generate new vectors for new iterations or you'll be recomputing same
// value over and over. No problem in changing data structures used by the algorithm (currently its a struct
// but that can change if you want to)


fn create_matrice(seed: u64) -> Array2<Fq> {
    let ref mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut matrice = Array2::<Fq>::default((200, 200));
    for i in 0..200{
        for j in 0..200{
            matrice[[i, j]] = Fq::rand(rng);
        }
    }
    matrice
}
fn get_square(matrice: &Array2<Fq>) -> Array2<Fq>{
    matrice.dot(matrice)
}

// You can either do a test on main or just remove main function and rename this file to lib.rs to remove the
// warning of not having a main implementation
fn main() {
    let matrix_a = create_matrice(1);
    let a_squared = get_square(&matrix_a);
    let matrix_b = create_matrice(2);
    let supposed_ab = matrix_a.clone().dot(&matrix_b);
    // print a and a^2
    let freivald = Freivald::new(supposed_ab.nrows());
    let verified = freivald.verify(&matrix_a, &matrix_b, &supposed_ab);
    println!("A*B=AB verified: {}", verified);

    let verified_once = Freivald::verify_once(&matrix_a, &matrix_a.clone(), &a_squared);
    println!("A*A=A_Squared verified using verify once: {}", verified_once);
    println!("A*A=A_Squared verified using verify: {}", freivald.verify(&matrix_a, &matrix_a, &a_squared));
}

// DONE TODO: Add proper types to input matrices. Remember matrices should hold Fq values
pub fn check_matrix_dimensions(matrix_a: &Array2<Fq>, matrix_b: &Array2<Fq>, supposed_ab: &Array2<Fq>) -> bool {
    // DONE TODO: Check if dimensions of making matrix_a * matrix_b matches values in supposed_ab.
    // If it doesn't you know its not the correct result independently of matrix contents
    matrix_a.nrows() == supposed_ab.nrows() && matrix_b.nrows() == supposed_ab.nrows() && matrix_a.nrows() == matrix_b.nrows()
    && matrix_a.ncols() == supposed_ab.ncols() && matrix_b.ncols() == supposed_ab.ncols() && matrix_a.ncols() == matrix_b.ncols()
}

#[cfg(test)]
mod tests {
    // #[macro_use]
    use lazy_static::lazy_static;
    use rstest::rstest;

    use super::*;
    lazy_static! {

        //todo!("add matrices types and values")
        static ref MATRIX_A: Array2<Fq> = create_matrice(1);
        static ref MATRIX_A_DOT_A: Array2<Fq> = get_square(&MATRIX_A);
        static ref MATRIX_B: Array2<Fq> = create_matrice(2);
        static ref MATRIX_B_DOT_B: Array2<Fq> = get_square(&MATRIX_B);
        static ref MATRIX_C: Array2<Fq> = create_matrice(3);
        static ref MATRIX_C_DOT_C: Array2<Fq> = get_square(&MATRIX_C);
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_C, &MATRIX_C_DOT_C)]
    fn freivald_verify_success_test(
        #[case] matrix_a: &Array2<Fq>,
        #[case] matrix_b: &Array2<Fq>,
        #[case] supposed_ab: &Array2<Fq>,
    ) {
        let freivald = Freivald::new(supposed_ab.nrows());
        assert!(freivald.verify(&matrix_a, &matrix_b, &supposed_ab));
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_B, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_A, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_B, &MATRIX_C_DOT_C)]
    fn freivald_verify_fail_test(
        #[case] a: &Array2<Fq>,
        #[case] b: &Array2<Fq>,
        #[case] c: &Array2<Fq>,
    ) {
        let freivald = Freivald::new(c.nrows());
        assert!(!freivald.verify(a, b, c));
    }
}
