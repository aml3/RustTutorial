extern mod extra;
use extra::arc::RWArc;

fn main() {
    let nums = [1,78,3,5,-2,5,7,-11];

    let numArc = RWArc::new(nums);

    for i in range(0, nums.len()) {
        let (port, chan)  = Chan::new();
        chan.send(numArc.clone());
        spawn(proc() {
            let taskArc = port.recv();
            taskArc.read(|taskNums| {
                    println!("{:d}", taskNums[i]);
                });
        });
    }
}

