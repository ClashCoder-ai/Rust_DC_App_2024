struct BonusTracker {
    next_2_overs_double: bool,
    rest_of_this_over_double: bool,
    next_over_double: bool,
    next_over_no_bonus: bool,
    balls_in_current_over: Vec<i32>, 
    current_over_bonus_multiplier: i32, 
    total_score: i32, 
}

impl BonusTracker {
    fn new() -> BonusTracker {
        BonusTracker {
            next_2_overs_double: false,
            rest_of_this_over_double: false,
            next_over_double: false,
            next_over_no_bonus: false,
            balls_in_current_over: Vec::new(),
            current_over_bonus_multiplier: 1,
            total_score: 0,
        }
    }

    fn bat(&mut self, runs: i32) {
        
        let run_display = if runs >= 4 { format!("*{}", runs) } else { format!("{}", runs) };

        let bonus_run_display = if self.current_over_bonus_multiplier > 1 {
            format!("{}x{}", run_display, self.current_over_bonus_multiplier)
        } else {
            run_display
        };

        
        let actual_runs = runs * self.current_over_bonus_multiplier;
        self.total_score += actual_runs;

        
        self.balls_in_current_over.push(runs);

        
        println!("Ball: {} | Runs: {}", self.balls_in_current_over.len(), bonus_run_display);

        if self.balls_in_current_over.len() == 6 {
            self.end_over();
        }
    }

    fn end_over(&mut self) {
        println!("Over completed: {:?}", self.balls_in_current_over);
        println!("Current total score: {}", self.total_score);

        
        self.apply_twists();

        
        self.balls_in_current_over.clear();
        self.current_over_bonus_multiplier = 1;

        println!("---");
    }

    fn apply_twists(&mut self) {
        let over_runs = &self.balls_in_current_over;

      
        if over_runs.iter().all(|&run| run >= 4) {
            println!("All balls were boundaries! Next two overs will have doubled runs.");
            self.next_2_overs_double = true;
        }

        if over_runs[0] == 6 && over_runs[1] == 6 && over_runs.iter().skip(2).all(|&run| run < 4) {
            println!("First two balls were sixes! Remaining runs in this over will be doubled.");
            self.current_over_bonus_multiplier = 2;
        }

        
        if over_runs[4] == 6 && over_runs[5] == 6 && over_runs.iter().take(4).all(|&run| run < 4) {
            println!("Last two balls were sixes! Runs in the next over will be doubled.");
            self.next_over_double = true;
        }

        
        if over_runs.iter().sum::<i32>() == 0 {
            println!("Maiden over! No bonuses will apply to the next over.");
            self.next_over_no_bonus = true;
        }

        
        if self.next_2_overs_double {
            self.current_over_bonus_multiplier = 2;
            self.next_2_overs_double = false;
        } else if self.next_over_double {
            self.current_over_bonus_multiplier = 2;
            self.next_over_double = false;
        }

       
        if self.next_over_no_bonus {
            self.current_over_bonus_multiplier = 1;
            self.next_over_no_bonus = false;
        }
    }
}

fn main() {
    let mut bonus_tracker = BonusTracker::new();

 
    bonus_tracker.bat(5);
    bonus_tracker.bat(4);
    bonus_tracker.bat(2);
    bonus_tracker.bat(1);
    bonus_tracker.bat(4);
    bonus_tracker.bat(3);

    
    bonus_tracker.bat(5);
    bonus_tracker.bat(4);
    bonus_tracker.bat(6);
    bonus_tracker.bat(4);
    bonus_tracker.bat(4);
    bonus_tracker.bat(6);

    
}


