use std::{fmt, sync::{Mutex, Arc}, thread, time::Instant};

// Global config
const N_NUMS : usize = 7;
const OPS : [Op; 4] = [Op::Add, Op::Sub, Op::Mul, Op::Div];
type TargetSize = i64;
type IntermediateType = f64;

// Data type to represent the 
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Op {
    Add, 
    Sub,
    Mul,
    Div
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
        }
    }
}
impl Op {
    fn compute(&self, x: IntermediateType, y: TargetSize) -> IntermediateType {
        match self {
            Op::Add => x + y as IntermediateType,
            Op::Sub => x - y as IntermediateType,
            Op::Mul => x * y as IntermediateType,
            Op::Div => x / y as IntermediateType,
            
        }
    }
}

#[derive(Debug, Clone)]
struct Result {
    acc: f64,
    used_nums: [TargetSize; N_NUMS],
    used_ops: [Op; N_NUMS-1],
    pos: usize,
    // unused_nums: Vec<TargetSize>,
}
impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = self.result();
        match res {
            Some(res) => {
                let mut str = res.to_string() + "=" + self.used_nums[0].to_string().as_str();
                for i in 1..self.pos {
                    str = str + self.used_ops[i-1].to_string().as_str();
                    str = str + self.used_nums[i].to_string().as_str();
                }
                write!(f, "{}", str)
            }
            None => write!(f, "Invalid result")
        }
    }
}
impl Result {
     fn new(first: TargetSize) -> Result {
        // Setup result
        let mut result = Result {
            acc: 0.0,
            used_nums: [0; N_NUMS],
            used_ops: [Op::Add; N_NUMS-1],
            pos: 0,
        };

        // Add in first value
        result.acc = first as IntermediateType;
        result.used_nums[0] = first;
        result.pos = 1;

        return result;
    }
    fn acc(&mut self, op: Op, n: TargetSize) {
        self.acc = op.compute(self.acc, n);
        self.used_nums[self.pos] = n;
        self.used_ops[self.pos-1] = op;
        self.pos += 1;
    }
    fn result(&self) -> Option<TargetSize> {
        if self.acc.fract() == 0.0 {
            return Some(self.acc.trunc() as TargetSize);
        }
        None
    }
}

fn main() {

    // let targets: Mutex<Vec<TargetSize>> = Mutex::new(vec![1096903,10716,5692214313,1299,561928800,11564,73759,39370,7253,17823991,37614,25023,234356907,670,-1547,529,-115,8296,22155,214387,-518,10920,170319,95868814,703157,447,-651852,34591,2966914,5214,-345,55,193270,114839,19323,224716500,177,15214787,270309,-55149,2082,49085150,15237000,19125,312631,3194,14554,18290000,3136,816,32007,818121,9521288,5166978,3428,3231,567,-56,-127703,305903,27165,5692213791,265023,12091,236,46771,25096,-22109,165,63,-4019925,159,15238830,220,180404,241,2100,18425,-59,81,305995,5088544,-673466,122430,442314,10212,93,-108818,-2790,137547,922,4102974,2374,14,7098,8118,161,-669,103850,2576254]);
    // let nums: [TargetSize; 5] = [42,67,86,21,74];

    // let targets = Arc::new(Mutex::new(vec![230641,-35919,4097227,-11953,2736,545420048,56598321,193,81301,76274432,2131686,-6868,236267,199,2152,2433910206,3027,10336,-2430437,604176,545,73,143983,20356522,5394480,3254892,-105,279725,150840,-14186,-4524,-3112,-87790,7380,246647,1770298,958,-26,486998,-75264,102408,-216943,4202,69825,-53,615537,2590,81,76563,6526,-1058,15690864,149969,-5230,240175,105,8933,104965,7584,572869,107,30481,244416000,-1078,34654,185760783,12596,13085631,7744,4086,-59,636168,34807,89,11029,4304,427013220,20531,32830,386750185,-2133230,-84849336,25618741,1309,134335,451864,-261526,9506,-9,19873,112673,-1488574,1245170,206440,8988,-4288,372,732212,728939,45890890]));
    // let nums = vec![67,39,82,43,12,95,19];

    let targets = Arc::new(Mutex::new(vec![74382,18672,24232,5926509,750,9349767,551,9557,7414,1100,158,841254317,-12428,-2039,217172,-2850,-304779,4814961,-71170,2,1211,98112,137076,13286,29682,10737,33526,998,52743,302271,1549,17534502,-13780491,8513,-4410432,1827264,44276544,-55499418,-37104,3745,-13432,2048,94,-4351968,15687,21,106,162279,17891,66532,9246399,54824,6974,11340,736,48,7686,-1171,10720,402876,472752,13917,12660,138,15066,2695,2763,19,791,6681,226702,970,-253470,6142293,-353685,10494042,426469212,4561596,589,-187148,5079,717999,-16167,995,18031,606455,163286,-107,661,120,356818176,117801225,-1797543,4561717,10433280,114,303666,73970,22860,445]));
    let nums = vec![57,48,9,73,6,19,78];
    
    // let nums = vec![1,2,3,4,5,6,7,8,9,10,11,12,13];

    let now =  Instant::now();

    let results = find_all(nums, targets);

    let elapsed_time = now.elapsed();
    for result in results {
        println!("{}", result);
    }
    println!("Results computed in {} milliseconds.", elapsed_time.as_millis());
}

fn find_all(numbers: Vec<TargetSize>, targets: Arc<Mutex<Vec<TargetSize>>>) -> Vec<Result> {
    
    let mut results = Vec::new();
    let mut handles = Vec::new();
    for num in numbers.clone() {
        let targets = targets.clone();
        let mut numbers = numbers.clone();
        numbers.retain(|&x| x != num);
        handles.push(thread::spawn(move || {
            let res = Result::new(num);
            next(res, numbers, &*targets)
        }));
    }

    for handle in handles {
        results.append(&mut handle.join().unwrap());
    }


    return results;
}

fn next(curr: Result, next_vals: Vec<TargetSize>, targets: &Mutex<Vec<TargetSize>>) -> Vec<Result> {


    let mut results = Vec::new();

    for op in OPS {
        for next_val in &next_vals {
            let mut res = curr.clone();
            let mut next_vals = next_vals.clone();
            next_vals.retain(|&x| x != *next_val);
            res.acc(op, *next_val);
            if res.pos == N_NUMS {
                match res.result() {
                    Some(val) => {
                        // println!("{}", res);
                        let mut targets = targets.lock().unwrap();
                        if targets.contains(&val) {
                            // println!("{}", res);
                            targets.retain(|&x| x != val);
                            results.push(res);
                        }
                    }
                    None => ()
                }
            } else {
                results.append(&mut next(res, next_vals.clone(), &targets));
            }

        }
    }

    return results;
}
