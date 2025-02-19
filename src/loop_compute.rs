use crate::loop_parser::{MagneticLoopMeasurement, MagneticLoopStack};

pub fn average_loop_stack(magnetic_loop_stack: &MagneticLoopStack) -> Vec<MagneticLoopMeasurement> {
    let mut average_loop: Vec<MagneticLoopMeasurement> = vec![
        MagneticLoopMeasurement {
            b: 0.0,
            bx: 0.0,
            by: 0.0,
            bz: 0.0,
            mx: 0.0,
            my: 0.0,
            mz: 0.0,
            ms: 0.0,
            vol: 0.0,
        };
        magnetic_loop_stack
            .steps_per_hysteresis_loop
    ];

    let loop_stack = &magnetic_loop_stack.hysteresis_loops;

    let n = loop_stack.len() as f64;

    for i in 0..loop_stack.len() {
        let current_loop = &loop_stack[i];
        for j in 0..current_loop.len() {
            average_loop[j].b += current_loop[j].b;
            average_loop[j].bx += current_loop[j].bx;
            average_loop[j].by += current_loop[j].by;
            average_loop[j].bz += current_loop[j].bz;
            average_loop[j].mx += current_loop[j].mx;
            average_loop[j].my += current_loop[j].my;
            average_loop[j].mz += current_loop[j].mz;
            average_loop[j].ms += current_loop[j].ms;
            average_loop[j].vol += current_loop[j].vol;
        }
    }

    for i in 0..magnetic_loop_stack.steps_per_hysteresis_loop {
        average_loop[i].b /= n;
        average_loop[i].bx /= n;
        average_loop[i].by /= n;
        average_loop[i].bz /= n;
        average_loop[i].mx /= n;
        average_loop[i].my /= n;
        average_loop[i].mz /= n;
        average_loop[i].ms /= n;
        average_loop[i].vol /= n;
    }

    average_loop
}

#[cfg(test)]
mod tests {
    use crate::loop_compute::average_loop_stack;
    use crate::loop_parser::{read_loop_files, MagneticLoopMeasurement};
    use std::path::Path;

    #[test]
    fn test_average_loop_stack() {
        let root_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("basic_loop_file");
        let basic1_loop: String = root_file_path
            .join("basic1.loop")
            .to_str()
            .unwrap()
            .to_string();
        let basic2_loop: String = root_file_path
            .join("basic2.loop")
            .to_str()
            .unwrap()
            .to_string();

        let file_names: Vec<String> = vec![basic1_loop, basic2_loop];

        let loop_stack = read_loop_files(&file_names).unwrap();

        let average_loop_stack = average_loop_stack(&loop_stack);

        let expected_data = vec![
            MagneticLoopMeasurement {
                b: 0.2,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.5878947549999997e-16,
                my: 2.786227635e-17,
                mz: -1.237348825e-16,
                ms: 476799.747,
                vol: 6.14125004e-22,
            },
            MagneticLoopMeasurement {
                b: 0.1999,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.587768085e-16,
                my: 2.7827558e-17,
                mz: -1.237681225e-16,
                ms: 476799.747,
                vol: 6.14125004e-22,
            },
            MagneticLoopMeasurement {
                b: 0.1998,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.587733855e-16,
                my: 2.782697045e-17,
                mz: -1.23774246e-16,
                ms: 476799.747,
                vol: 6.14125004e-22,
            },
            MagneticLoopMeasurement {
                b: 0.1997,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.5877539050000004e-16,
                my: 2.787937485e-17,
                mz: -1.2374536499999999e-16,
                ms: 476799.747,
                vol: 6.14125004e-22,
            },
            MagneticLoopMeasurement {
                b: 0.1996,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.58766866e-16,
                my: 2.78963268e-17,
                mz: -1.2375200700000001e-16,
                ms: 476799.747,
                vol: 6.14125004e-22,
            },
            MagneticLoopMeasurement {
                b: 0.1995,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.5875984199999996e-16,
                my: 2.7889545249999996e-17,
                mz: -1.237638395e-16,
                ms: 476799.747,
                vol: 6.14125004e-22,
            },
            MagneticLoopMeasurement {
                b: 0.1994,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.58758458e-16,
                my: 2.78733762e-17,
                mz: -1.2376765500000002e-16,
                ms: 476799.747,
                vol: 6.14125004e-22,
            },
            MagneticLoopMeasurement {
                b: 0.1993,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.587625705e-16,
                my: 2.7853663949999997e-17,
                mz: -1.23758808e-16,
                ms: 476799.747,
                vol: 6.14125004e-22,
            },
            MagneticLoopMeasurement {
                b: 0.1992,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.5876077849999997e-16,
                my: 2.787386995e-17,
                mz: -1.237507455e-16,
                ms: 476799.747,
                vol: 6.14125004e-22,
            },
        ];

        for i in 0..average_loop_stack.len() {
            assert!((expected_data[i].b - average_loop_stack[i].b).abs() < 1e-12);
            assert!((expected_data[i].bx - average_loop_stack[i].bx).abs() < 1e-12);
            assert!((expected_data[i].by - average_loop_stack[i].by).abs() < 1e-12);
            assert!((expected_data[i].bz - average_loop_stack[i].bz).abs() < 1e-12);
            assert!((expected_data[i].mx - average_loop_stack[i].mx).abs() < 1e-12);
            assert!((expected_data[i].my - average_loop_stack[i].my).abs() < 1e-12);
            assert!((expected_data[i].mz - average_loop_stack[i].mz).abs() < 1e-12);
            assert!((expected_data[i].ms - average_loop_stack[i].ms).abs() < 1e-12);
            assert!((expected_data[i].vol - average_loop_stack[i].vol).abs() < 1e-12);
        }
    }

}
