#[cfg(test)]
//#[macro_use]

//extern crate quickcheck;

use super::super::matrix;

#[test]
fn test_reverse() {
    let verify = |a: matrix::Matrix44| {
        let inv_a = a.inverse();
        assert_ne!(a.det(), 0.0);
        assert_eq!((a * inv_a).epsilon_normalized(None), super::super::matrix::Matrix44::identity());
    };

    verify(matrix::Matrix44 {
        // {{1,2,3,4},{3,9,3,5},{5,2,3,4},{7,2,12,4}}
        elements: [
            [1.0, 2.0, 3.0, 4.0],
            [3.0, 9.0, 3.0, 5.0],
            [5.0, 2.0, 3.0, 4.0],
            [7.0, 2.0, 12.0, 4.0]]
    });

    verify(matrix::Matrix44 {
        // {{1,2,3,4},{3,9,3,5},{5,2,3,4},{7,2,12,4}}
        elements: [
            [1.0, 2.0, 3.0, 4.0],
            [12.0, 13.0, 14.0, 5.0],
            [11.0, 16.0, 15.0, 6.0],
            [10.0, 9.0, 8.0, 7.0]]
    });
}