extern mod extra;
use extra::arc::RWArc;

fn main() {
    let num = 0;
    let numArc = RWArc::new(num);

    for i in range(0, 50000) {
        let (port, chan)  = Chan::new();
        chan.send(numArc.clone());
        do spawn {
            let taskArc = port.recv();
            let mut newNum = 0;
            taskArc.write(|taskNum| {
                *taskNum += 1;
                newNum = *taskNum;
            });
            let collatzN = collatz(newNum);
            println!("Collatz of {:d} = {:d}", newNum, collatzN);
        }
    }
}
fn collatz(N: int) -> int {
    let mut nLoc = N;
    let mut out = 0;
    while (nLoc != 1) {
        out += 1;
        match nLoc % 2 {
	    0 => {nLoc = nLoc/2;}
	    _ => {nLoc = nLoc*3+1; }
	}
    }
    return out;
}