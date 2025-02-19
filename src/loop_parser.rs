use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub struct MagneticLoopMeasurement {
    pub b: f64,
    pub bx: f64,
    pub by: f64,
    pub bz: f64,
    pub mx: f64,
    pub my: f64,
    pub mz: f64,
    pub ms: f64,
    pub vol: f64,
}

#[derive(Debug)]
pub struct MagneticLoopStack {
    pub hysteresis_loops: Vec<Vec<MagneticLoopMeasurement>>,
    pub no_of_hysteresis_loops: usize,
    pub steps_per_hysteresis_loop: usize,
    pub field_start: f64,
    pub field_end: f64,
    pub field_step: f64
}

pub fn read_loop_file(file_name: &str) -> io::Result<Vec<MagneticLoopMeasurement>> {
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);
    let mut data: Vec<MagneticLoopMeasurement> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        // Skip header
        if i == 0 {
            continue;
        }

        // Parse line
        let values: Vec<f64> = line
            .split(",")
            .map(|s| s.trim().parse::<f64>())
            .filter_map(Result::ok)
            .collect();

        // Ensure that 9 values exist
        if values.len() == 9 {
            data.push(MagneticLoopMeasurement{
                b: values[0],
                bx: values[1],
                by: values[2],
                bz: values[3],
                mx: values[4],
                my: values[5],
                mz: values[6],
                ms: values[7],
                vol: values[8],
            });
        }
    }

    Ok(data)
}

pub fn read_loop_files(file_names: &Vec<String>) -> io::Result<MagneticLoopStack> {
    let mut loop_stack: Vec<Vec<MagneticLoopMeasurement>> = Vec::new();
    for file_name in file_names {
        let file_data = read_loop_file(file_name)?;
        loop_stack.push(file_data);
    }

    // Grab the first loop from our loop-stack.
    let first_loop = match loop_stack.first() {
        Some(first_vec) => first_vec,
        None => return Err(io::Error::new(io::ErrorKind::InvalidData, "No loop files found.")),
    };

    // Check that each hysteresis loop has the same number of entries.
    let loop_length = first_loop.len();
    for i in 0..loop_stack.len() {
        if loop_stack[i].len() != loop_length {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidData, "Loop files are not the same length."))
        }
    }

    // Check that each hysteresis loop has the same field step.
    let field_step = first_loop[1].b - first_loop[0].b;
    for i in 0..loop_stack.len() {
        let loop_data = &loop_stack[i];
        for j in 0..loop_data.len() - 1 {
            let sub_step = loop_data[j+1].b - loop_data[j].b;
            if (sub_step - field_step).abs() > 1e-12 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "A field step was too large"))
            };
        }
    }

    // Check that each hysteresis loop has the same start/end field.
    let field_start = first_loop[0].b;
    let field_end = first_loop[loop_length - 1].b;

    for i in 0..loop_stack.len() {
        if (loop_stack[i][0].b - field_start).abs() > 1e-12 {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Start field across loop files must be the same."))
        }
        if (field_end - loop_stack[i][loop_length - 1].b).abs() > 1e-12 {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "End field across loop files must be the same."))
        }
    }

    let no_of_hysteresis_loops = loop_stack.len();

    Ok(MagneticLoopStack{
        hysteresis_loops: loop_stack,
        no_of_hysteresis_loops,
        steps_per_hysteresis_loop: loop_length,
        field_start,
        field_end,
        field_step,
    })

}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::loop_parser::{read_loop_file, read_loop_files, MagneticLoopMeasurement};

    #[test]
    fn test_read_loop_file() {
        let root_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("basic_loop_file");

        let basic1_loop = root_file_path.join("basic1.loop");

        let data = read_loop_file(basic1_loop.to_str().unwrap()).unwrap();

        let expected_data: Vec<MagneticLoopMeasurement> = vec![
            MagneticLoopMeasurement {
                b: 0.2,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60122213e-16,
                my: 2.64811678e-17,
                mz: -1.2275386e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1999,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60127653e-16,
                my: 2.64428752e-17,
                mz: -1.22748051e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1998,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60093695e-16,
                my: 2.65155924e-17,
                mz: -1.22805125e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1997,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60119131e-16,
                my: 2.64734772e-17,
                mz: -1.22750862e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1996,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60116157e-16,
                my: 2.64826784e-17,
                mz: -1.22749988e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1995,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60100987e-16,
                my: 2.65203432e-17,
                mz: -1.22769301e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1994,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60101663e-16,
                my: 2.64647317e-17,
                mz: -1.22779807e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1993,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60104358e-16,
                my: 2.64736136e-17,
                mz: -1.22765499e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1992,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60101731e-16,
                my: 2.64752758e-17,
                mz: -1.22766567e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            }];

        assert_eq!(data.len(), expected_data.len());

        for i in 0..data.len() {
            assert!((expected_data[i].b - data[i].b).abs() < 1e-12);
            assert!((expected_data[i].bx - data[i].bx).abs() < 1e-12);
            assert!((expected_data[i].by - data[i].by).abs() < 1e-12);
            assert!((expected_data[i].bz - data[i].bz).abs() < 1e-12);
            assert!((expected_data[i].mx - data[i].mx).abs() < 1e-12);
            assert!((expected_data[i].my - data[i].my).abs() < 1e-12);
            assert!((expected_data[i].mz - data[i].mz).abs() < 1e-12);
            assert!((expected_data[i].ms - data[i].ms).abs() < 1e-12);
            assert!((expected_data[i].vol - data[i].vol).abs() < 1e-12);
        }
    }

    #[test]
    fn test_read_loop_files() {
        let root_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("basic_loop_file");
        let basic1_loop: String = root_file_path.join("basic1.loop").to_str().unwrap().to_string();
        let basic2_loop: String = root_file_path.join("basic2.loop").to_str().unwrap().to_string();

        let file_names: Vec<String> = vec![basic1_loop, basic2_loop];

        let data = read_loop_files(&file_names).unwrap();

        let expected_data_0: Vec<MagneticLoopMeasurement> = vec![
            MagneticLoopMeasurement {
                b: 0.2,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60122213e-16,
                my: 2.64811678e-17,
                mz: -1.2275386e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1999,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60127653e-16,
                my: 2.64428752e-17,
                mz: -1.22748051e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1998,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60093695e-16,
                my: 2.65155924e-17,
                mz: -1.22805125e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1997,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60119131e-16,
                my: 2.64734772e-17,
                mz: -1.22750862e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1996,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60116157e-16,
                my: 2.64826784e-17,
                mz: -1.22749988e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1995,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60100987e-16,
                my: 2.65203432e-17,
                mz: -1.22769301e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1994,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60101663e-16,
                my: 2.64647317e-17,
                mz: -1.22779807e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1993,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60104358e-16,
                my: 2.64736136e-17,
                mz: -1.22765499e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1992,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.60101731e-16,
                my: 2.64752758e-17,
                mz: -1.22766567e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            }];

        let expected_data_1: Vec<MagneticLoopMeasurement> = vec![
            MagneticLoopMeasurement {
                b: 0.2,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.57456738e-16,
                my: 2.92433849e-17,
                mz: -1.24715905e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1999,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.57425964e-16,
                my: 2.92122408e-17,
                mz: -1.24788194e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1998,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.57453076e-16,
                my: 2.91383485e-17,
                mz: -1.24743367e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1997,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.5743165e-16,
                my: 2.92852725e-17,
                mz: -1.24739868e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1996,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.57417575e-16,
                my: 2.93099752e-17,
                mz: -1.24754026e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1995,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.57418697e-16,
                my: 2.92587473e-17,
                mz: -1.24758378e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1994,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.57415253e-16,
                my: 2.92820207e-17,
                mz: -1.24755503e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1993,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.57420783e-16,
                my: 2.92337143e-17,
                mz: -1.24752117e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
            MagneticLoopMeasurement {
                b: 0.1992,
                bx: 0.918615235,
                by: 0.0761053945,
                bz: -0.387755102,
                mx: 2.57419826e-16,
                my: 2.92724641e-17,
                mz: -1.24734924e-16,
                ms: 476799.747,
                vol: 6.14125004e-22
            },
        ];

        assert!(data.no_of_hysteresis_loops == 2);

        assert!((data.field_start - 0.2).abs() < 1e-12);
        assert!((data.field_end - 0.1992).abs() < 1e-12);
        assert!((data.field_step - (-0.0001)).abs() < 1e-12);

        for i in 0..data.steps_per_hysteresis_loop {
            assert!((expected_data_0[i].b - data.hysteresis_loops[0][i].b).abs() < 1e-12);
            assert!((expected_data_0[i].bx - data.hysteresis_loops[0][i].bx).abs() < 1e-12);
            assert!((expected_data_0[i].by - data.hysteresis_loops[0][i].by).abs() < 1e-12);
            assert!((expected_data_0[i].bz - data.hysteresis_loops[0][i].bz).abs() < 1e-12);
            assert!((expected_data_0[i].mx - data.hysteresis_loops[0][i].mx).abs() < 1e-12);
            assert!((expected_data_0[i].my - data.hysteresis_loops[0][i].my).abs() < 1e-12);
            assert!((expected_data_0[i].mz - data.hysteresis_loops[0][i].mz).abs() < 1e-12);
            assert!((expected_data_0[i].ms - data.hysteresis_loops[0][i].ms).abs() < 1e-12);
            assert!((expected_data_0[i].vol - data.hysteresis_loops[0][i].vol).abs() < 1e-12);
        }

        for i in 0..data.steps_per_hysteresis_loop {
            assert!((expected_data_1[i].b - data.hysteresis_loops[1][i].b).abs() < 1e-12);
            assert!((expected_data_1[i].bx - data.hysteresis_loops[1][i].bx).abs() < 1e-12);
            assert!((expected_data_1[i].by - data.hysteresis_loops[1][i].by).abs() < 1e-12);
            assert!((expected_data_1[i].bz - data.hysteresis_loops[1][i].bz).abs() < 1e-12);
            assert!((expected_data_1[i].mx - data.hysteresis_loops[1][i].mx).abs() < 1e-12);
            assert!((expected_data_1[i].my - data.hysteresis_loops[1][i].my).abs() < 1e-12);
            assert!((expected_data_1[i].mz - data.hysteresis_loops[1][i].mz).abs() < 1e-12);
            assert!((expected_data_1[i].ms - data.hysteresis_loops[1][i].ms).abs() < 1e-12);
            assert!((expected_data_1[i].vol - data.hysteresis_loops[1][i].vol).abs() < 1e-12);
        }
    }

    #[test]
    fn test_read_loop_files_not_same_length() {
        let root_file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("basic_loop_file");
        let basic1_loop: String = root_file_path.join("basic1.loop").to_str().unwrap().to_string();
        let basic2_loop: String = root_file_path.join("basic2.loop").to_str().unwrap().to_string();
        let basic3_loop: String = root_file_path.join("basic3.loop").to_str().unwrap().to_string();

        let file_names: Vec<String> = vec![basic1_loop, basic2_loop, basic3_loop];

        assert!(read_loop_files(&file_names).is_err());
    }

}