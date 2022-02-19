use super::Record;

pub struct Classifier {
    pub name: String,
    training_data: Vec<Record>,
    algorithm: fn(input: u8, test: u8) -> i64,
}

impl Classifier {
    pub fn predict<'a>(&self, input: &'a Record) -> (Record, Record) {
        let mut best_total = i64::MAX;
        let mut best = &Record {actual: 0, image: [0; 784]};
        for item in self.training_data.iter() {
            let mut total: i64 = 0;
            for i in 0..783 {
                let diff = (self.algorithm)(input.image[i], item.image[i]);
                total += diff;
            }
            if total < best_total {
                best_total = total;
                best = item;
            }
        }
    
        (input.clone(), best.clone())    
    }
}

pub fn get_manhattan_classifier(training_data: Vec<Record>) -> Classifier {
    Classifier {
        name: "Manhattan Classifier".to_string(),
        training_data,
        algorithm: manhattan_algorithm,
    }
}

fn manhattan_algorithm(input: u8, test: u8) -> i64 {
    ((i64::from(input))-(i64::from(test))).abs()
}

pub fn get_euclidean_classifier(training_data: Vec<Record>) -> Classifier {
    Classifier {
        name: "Euclidean Classifier".to_string(),
        training_data,
        algorithm: euclidean_algorithm,
    }
}

fn euclidean_algorithm(input: u8, test: u8) -> i64 {
    let diff = (i64::from(input))-(i64::from(test));
    diff * diff
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    fn get_test_training_set() -> Vec<Record> {
        vec![
            Record {
                actual: 5,
                image: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,41,149,156,179,254,254,201,119,46,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,13,147,241,253,253,254,253,253,253,253,245,160,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,31,224,253,253,180,174,175,174,174,174,174,223,247,145,6,0,0,0,0,0,0,0,0,0,0,0,0,7,197,254,253,165,2,0,0,0,0,0,0,12,102,184,16,0,0,0,0,0,0,0,0,0,0,0,0,152,253,254,162,18,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,235,254,158,15,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,74,250,253,15,0,0,0,16,20,19,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,199,253,253,0,0,25,130,235,254,247,145,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,20,253,253,177,100,219,240,253,253,254,253,253,125,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,193,253,253,254,253,253,200,155,155,238,253,229,23,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,61,249,254,241,150,30,0,0,0,215,254,254,58,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,36,39,30,0,0,0,0,0,214,253,234,31,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,41,241,253,183,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,201,253,253,102,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,114,254,253,154,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,62,254,255,241,30,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,10,118,235,253,249,103,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,55,81,0,102,211,253,253,253,135,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,79,243,234,254,253,253,216,117,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,48,245,253,254,207,126,27,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            },
            Record {
                actual: 3,
                image: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,60,136,136,147,254,255,199,111,18,9,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,25,152,253,253,253,253,253,253,253,253,253,124,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,135,225,244,253,202,200,181,164,216,253,253,211,151,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,30,149,78,3,0,0,0,20,134,253,253,224,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,28,206,253,253,224,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,78,253,253,253,224,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,99,234,253,253,224,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,14,142,220,219,236,253,253,240,121,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,24,253,253,253,253,235,233,253,253,185,53,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,8,150,194,194,194,53,40,97,253,253,170,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,122,253,253,170,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,55,237,253,253,170,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,130,253,253,253,170,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,12,120,193,253,253,214,28,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,153,253,253,253,253,212,30,0,0,0,0,0,0,0,0,0,0,0,0,0,33,136,70,6,0,27,67,186,253,253,253,253,234,31,0,0,0,0,0,0,0,0,0,0,0,0,0,26,231,253,253,191,183,223,253,253,253,253,172,216,112,0,0,0,0,0,0,0,0,0,0,0,0,0,0,36,215,253,253,253,253,253,253,253,253,253,47,25,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,87,223,253,253,253,244,152,223,223,109,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,67,50,176,148,78,16,0,12,12,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            },
            Record {
                actual: 8,
                image: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,24,24,97,253,253,253,253,255,180,48,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,30,186,252,252,253,252,252,252,252,253,252,227,29,0,0,0,0,0,0,0,0,0,0,0,0,0,0,38,155,252,252,252,253,252,252,227,79,222,252,252,129,0,0,0,0,0,0,0,0,0,0,0,0,0,85,233,252,252,252,252,253,252,252,202,11,180,252,252,119,0,0,0,0,0,0,0,0,0,0,0,0,43,240,253,252,252,252,252,253,252,252,244,126,201,252,252,150,0,0,0,0,0,0,0,0,0,0,0,7,212,253,255,253,253,253,232,221,42,0,104,253,255,253,205,21,0,0,0,0,0,0,0,0,0,0,0,25,223,252,253,252,252,214,18,0,0,34,215,252,253,223,56,0,0,0,0,0,0,0,0,0,0,0,0,0,99,246,253,252,252,77,0,7,70,203,252,252,173,25,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,42,253,252,252,236,103,160,252,252,218,108,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,148,252,252,252,252,253,231,106,14,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,24,253,253,253,253,255,159,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,43,118,252,240,244,252,253,231,37,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,19,164,246,253,187,50,99,246,253,252,69,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,80,232,252,203,58,0,0,135,253,252,121,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,43,246,252,200,11,0,0,0,116,253,252,69,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,162,253,192,11,0,0,0,0,179,255,253,69,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,178,252,119,0,5,47,47,140,244,253,252,69,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,186,252,227,184,191,252,252,252,252,253,240,50,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,11,144,227,252,252,253,252,252,252,252,98,37,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,48,137,242,253,231,137,137,32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            },
            Record {
                actual: 9,
                image: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,15,48,143,186,244,143,31,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,83,209,253,252,252,252,252,192,15,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,166,241,252,253,252,170,162,252,252,113,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,61,234,252,252,243,121,44,2,21,245,252,122,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,80,252,252,243,163,50,0,0,0,5,101,88,8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,105,234,252,210,88,0,0,0,0,74,199,240,43,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,185,252,210,21,0,4,12,41,231,249,252,252,55,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,242,252,218,154,154,184,252,253,252,252,248,184,22,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,209,252,252,252,252,252,252,253,252,252,196,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,17,57,142,95,142,61,81,253,252,209,20,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,11,177,255,230,86,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,12,124,252,245,57,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,135,252,252,86,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,79,248,252,233,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,231,252,202,12,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,20,175,248,252,136,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,109,252,252,159,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,33,218,252,252,192,141,14,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,132,252,252,252,205,74,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,132,252,252,146,13,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            },
            Record {
                actual: 1,
                image: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,169,207,33,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,168,254,105,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,84,249,254,105,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,89,254,254,105,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,89,254,193,14,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,89,254,184,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,204,254,184,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,89,254,184,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,13,209,254,178,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,37,209,254,254,69,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,107,254,254,254,184,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,187,254,254,134,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,185,254,155,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,185,254,238,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,185,254,254,8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,185,254,231,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,185,255,87,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,173,254,87,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,9,254,87,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,9,254,87,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            },
        ]
    }

    #[test]
    fn manhattan_predict_matches_identical() {
        let training = get_test_training_set();
        let input = training[0].clone();
        let classifier = get_manhattan_classifier(training);
        let (_, result) = classifier.predict(&input);
        assert_eq!(input.actual, result.actual);
    }

    #[test]
    fn euclidean_predict_matches_identical() {
        let training = get_test_training_set();
        let input = training[0].clone();
        let classifier = get_euclidean_classifier(training);
        let (_, result) = classifier.predict(&input);
        assert_eq!(input.actual, result.actual);
    }
}