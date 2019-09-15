use ndarray::array;
use ndarray::Array;

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;
    use test::Bencher;

    #[test]
    fn rdp_simple() {
        // from https://github.com/sebleier/RDP

        let the_points = arr2(&[
            [0.0, 0.0],
            [1.0, 0.0],
            [2.0, 0.0],
            [2.0, 1.0],
            [2.0, 2.0],
            [1.0, 2.0],
            [0.0, 2.0],
            [0.0, 1.0],
            [0.0, 0.0],
        ]);
        let indices = iter(&the_points, 1.0);
        let total = indices.iter().map(|&x| if x { 1 } else { 0 }).fold(0, |total, next| total + next);
        assert_eq!(total, 5);
        let final_points = mask(&the_points, &indices);
        assert_eq!(
            indices,
            vec![true, false, true, false, true, false, true, false, true]
        );
        assert_eq!(
            final_points,
            arr2(&[
                [0.0, 0.0],
                [2.0, 0.0],
                [2.0, 2.0],
                [0.0, 2.0],
                [0.0, 0.0],
            ])
        );
    }

    #[bench]
    fn bench_100_simple_rdp(b: &mut Bencher) {
        let the_points = arr2(&[
            [0.0, 0.0],
            [1.0, 0.0],
            [2.0, 0.0],
            [2.0, 1.0],
            [2.0, 2.0],
            [1.0, 2.0],
            [0.0, 2.0],
            [0.0, 1.0],
            [0.0, 0.0],
        ]);

        b.iter(|| {
            for _ in 0..100 {
                let n = test::black_box(&the_points);
                iter(n, 1.0);
            }
        });
    }

    #[test]
    fn test_norm_zero() {
        let pt1 = Array::from_vec(vec![0.0, 0.0]);
        let pt2 = Array::from_vec(vec![0.0, 0.0]);
        let actual = norm(&pt1.slice(s![..]), &pt2.slice(s![..]));
        assert_eq!(actual, 0.0);
    }

    #[test]
    fn test_norm_identical() {
        let pt1 = Array::from_vec(vec![-203.423, 9.323203]);
        let pt2 = Array::from_vec(vec![-203.423, 9.323203]);
        let actual = norm(&pt1.slice(s![..]), &pt2.slice(s![..]));
        assert_eq!(actual, 0.0);
    }

    #[test]
    fn test_norm_distance() {
        let pt1 = Array::from_vec(vec![51.0, 1.0]);
        let pt2 = Array::from_vec(vec![1.0, 1.0]);
        let actual = norm(&pt1.slice(s![..]), &pt2.slice(s![..]));
        assert_eq!(actual, 50.0);
    }
}

fn norm<'a>(
    point_01: &'a ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 1]>>,
    point_02: &'a ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 1]>>,
) -> f64 {
    let x = point_01[0] - point_02[0];
    let y = point_01[1] - point_02[1];
    (x * x + y * y).sqrt()
}

fn distance_segment<'a, 'b>(
    point: &'a ndarray::ArrayBase<ndarray::ViewRepr<&'a f64>, ndarray::Dim<[usize; 1]>>,
    start: &'b ndarray::ArrayBase<ndarray::ViewRepr<&'b f64>, ndarray::Dim<[usize; 1]>>,
    end: &'b ndarray::ArrayBase<ndarray::ViewRepr<&'b f64>, ndarray::Dim<[usize; 1]>>,
) -> f64 {
    // println!(" -=- Distance Function -=- ");
    // println!("point {}", point);
    // println!("start {}", start);
    // println!("end {}", end);

    if start.all_close(end, 1e-8) {
        // println!("diff is zero");
        return norm(point, start);
    }
    let segment = end - start;
    let point_start = point - start;
    let segment_sqr_length = (&segment * &segment).sum();
    // println!("(&point_start * &point_start) {}", (&segment * &segment));
    // println!("segment_sqr_length {}", segment_sqr_length);
    let t = (&segment * &point_start).sum() / segment_sqr_length;
    // println!("{}", t);
    let x = (t * segment) + start;

    let (near_pt_x, near_pt_y) = if t < 0.0 {
        (start[0], start[1])
    } else if t >= 1.0 {
        (end[0], end[1])
    } else {
        (x[0], x[1])
    };
    let near_pt = array![near_pt_x, near_pt_y];
    // println!("Near {}", near_pt);

    norm(point, &near_pt.slice(s![..]))
}

pub fn iter(
    points: &ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<ndarray::IxDynImpl>>,
    epsilon: f64,
) -> Vec<bool> {
    let point_count = points.shape()[0];

    let start_index = 0;
    let final_index = point_count - 1;

    let global_start_index = start_index;
    let mut indices = vec![true; final_index - start_index + 1];
    let mut stk = vec![(start_index, final_index)];

    while stk.len() > 0 {
        let (start, last) = stk.pop().expect("Unable to find the next indexes");
        // println!(" -==================- Top of Loop -==================-");
        // println!("Stk length {}", stk.len());
        // println!("Stk {:?}", stk);
        // println!("start {}, end {}", start, last);
        let start_point = &points.slice(s![start, ..]);
        let last_point = &points.slice(s![last, ..]);
        // println!("{} -> {}", start_point, last_point);
        let mut distance_max = 0.0;
        let mut index = start;

        // println!("Range {} -> {}", 1 + index, last);
        for i in (1 + index)..last {
            // println!("Working on {}", i);
            if indices[i - global_start_index] {
                // println!("Operating on {}", i - global_start_index);
                let point_test = &points.slice(s![i, ..]);
                let d = distance_segment(point_test, start_point, last_point);
                // println!("Distance {} -> {}", point_test, d);
                distance_max = if d > distance_max {
                    index = i;
                    d
                } else {
                    distance_max
                };
            }
        }
        // println!("Calculated Distance Max {}", distance_max);

        if distance_max > epsilon {
            stk.push((start, index));
            stk.push((index, last));
        } else {
            for i in (start + 1)..last {
                indices[i - global_start_index] = false;
            }
        }
        // println!("{:?}", indices);
        // println!("Stk length {} End", stk.len());
        // println!("Stk {:?}", stk);
    }

    indices
}

pub fn mask(
    points: &ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<ndarray::IxDynImpl>>,
    indices: &Vec<bool>,
) -> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> {
    // println!("{:?}", indices);
    let mut final_points: Vec<f64> = Vec::new();
    // let total = indices.iter().map(|&x| if x { 1 } else { 0 }).fold(0, |total, next| total + next);
    // println!("TOTAL: {}", total);
    // let mut result = Array::zeros((total, 2));
    // let mut pointer = 0;
    for idx in 0..points.shape()[0] {
        if indices[idx] {
            let point_test = &points.slice(s![idx, ..]);
            final_points.push(point_test[0]);
            final_points.push(point_test[1]);
        }
    }
    Array::from_shape_vec((final_points.len() / 2, 2), final_points).expect("Unable to build")
}
