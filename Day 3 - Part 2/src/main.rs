use std::collections::VecDeque;
use std::collections::HashMap;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Step {
    direction: Direction,
    num_of_cells: usize,
}

trait ConvertToStep {
    fn to_step(&self) -> Step;
}

impl ConvertToStep for str {
    fn to_step(&self) -> Step {
        if self.len() < 2 {
            panic!("Invalid step description.");
        }
    
        let mut chars = self.chars();
        
        let direction_char = chars.next().unwrap().to_ascii_lowercase();
        let direction = match direction_char {
            'l' => Direction::Left,
            'r' => Direction::Right,
            'u' => Direction::Up,
            'd' => Direction::Down,
             _  => panic!("Unknown direction found: '{}'.", direction_char)
        };
    
        let num_of_cells = chars.as_str().parse::<usize>().unwrap();
    
        Step{ direction, num_of_cells }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Cell {
    x: usize,
    y: usize,
}

struct Field {
    cells: VecDeque<VecDeque<u8>>,
    center: Cell,
    // A list of marker and their associated maps that save the cost of each visited cell.
    position_to_num_of_steps_list: Vec<(u8, HashMap<Cell, u64>)>,
}

impl Field {
    fn new(size: usize, fill_value: u8, center_value: u8) -> Field {
        if size == 0 {
            panic!("A field must have a size > 0.");
        }

        let mut field = Field {
            cells: vec![vec![fill_value; size].into_iter().collect(); size].into_iter().collect(),
            center: Cell{ x: 0, y: 0 },
            position_to_num_of_steps_list: Vec::new(),
        };
        field.cells[0][0] = center_value;
        field
    }
}

trait CanGrow {
    fn grow(&mut self, new_horizontal_size: i64, new_vertical_size: i64);
    fn grow_horizontally(&mut self, new_horizontal_size: i64);
    fn grow_vertically(&mut self, new_vertical_size: i64);
}

trait HandleLines {
    fn add_line(&mut self, line: &Vec<&str>, marker: u8);
    fn calculate_intersections(&self, marker: u8) -> Vec<Cell>;
    fn calculate_minimum_steps_to_reach_any_intersection(&self, marker: u8) -> u64;
}

impl CanGrow for Field {
    fn grow(&mut self, new_horizontal_size: i64, new_vertical_size: i64) {
        self.grow_horizontally(new_horizontal_size);
        self.grow_vertically(new_vertical_size);
    }

    fn grow_horizontally(&mut self, new_horizontal_size: i64) {
        let is_positive_growth = new_horizontal_size >= 0;
        
        let minimum_grow_size = 
            if is_positive_growth { new_horizontal_size.abs() + self.center.x as i64 - self.cells.len() as i64}
            else { new_horizontal_size.abs() - self.center.x as i64};

        // If the field shall grow, grow at least 100 cells.
        let relative_grow_size = 
            if      minimum_grow_size <= 0   { 0 }
            else if minimum_grow_size <= 100 { 100 as usize }
            else    { minimum_grow_size as usize };

        let current_vertical_size = self.cells[0].len();

        if is_positive_growth {
            for _ in 0..relative_grow_size {
                self.cells.push_back(VecDeque::with_capacity(current_vertical_size));
                self.cells.back_mut().unwrap().resize(current_vertical_size, 0);
            }
        }
        else {
            for _ in 0..relative_grow_size {
                self.cells.push_front(VecDeque::with_capacity(current_vertical_size));
                self.cells.front_mut().unwrap().resize(current_vertical_size, 0);
            }
        }

        // Move the center if the deque grew to the left.
        if !is_positive_growth {
            self.center.x += relative_grow_size as usize;
        }
    }
    
    fn grow_vertically(&mut self, new_vertical_size: i64) {
        let is_positive_growth = new_vertical_size >= 0;

        let minimum_grow_size = 
            if is_positive_growth { new_vertical_size.abs() + self.center.y as i64 - self.cells[0].len() as i64}
            else { new_vertical_size.abs() - self.center.y as i64 };

        // If the field shall grow, grow at least 100 cells.
        let relative_grow_size = 
            if      minimum_grow_size <= 0   { 0 }
            else if minimum_grow_size <= 100 { 100 as usize }
            else    { minimum_grow_size as usize };

        let current_vertical_size = self.cells[0].len();

        if is_positive_growth {
            for vertical_vec in self.cells.iter_mut() {
                vertical_vec.resize(current_vertical_size + relative_grow_size, 0);
            }
        }
        else {
            for vertical_vec in self.cells.iter_mut() {
                for _ in 0..relative_grow_size {
                    vertical_vec.push_front(0);
                }
            }
        }

        // Move the center if the deque grew to the top.
        if !is_positive_growth {
            self.center.y += relative_grow_size as usize;
        }
    }
}

impl HandleLines for Field {
    fn add_line(&mut self, line: &Vec<&str>, marker: u8) {
        // Check if the marker has already been added.
        if self.position_to_num_of_steps_list.iter().any(|marker_and_hash_map| marker_and_hash_map.0 == marker) {
            panic!("A line with the marker {} has already been added. No marker can be added twice.", marker);
        }

        let mut x: i64 = 0;
        let mut y: i64 = 0;
        let mut position_to_num_of_steps: HashMap<Cell, u64> = HashMap::new();
        let mut taken_steps: u64 = 0;

        //self.position_to_num_of_steps_list.push((marker, HashMap::new()));

        let get_cell = |x, y, center_x, center_y| -> Cell {
            Cell{ 
                x: (center_x as i64 + x) as usize, 
                y: (center_y as i64 + y) as usize 
            }
        };

        for &step_string in line.iter() {
            let step = step_string.to_step();

            match step.direction {
                Direction::Left  => self.grow_horizontally(x - step.num_of_cells as i64 - 1),
                Direction::Right => self.grow_horizontally(x + step.num_of_cells as i64 + 1),
                Direction::Down  => self.grow_vertically(y - step.num_of_cells as i64 - 1),
                Direction::Up    => self.grow_vertically(y + step.num_of_cells as i64 + 1),
            };

            for _ in 0..step.num_of_cells {
                match step.direction {
                    Direction::Left  => x -= 1,
                    Direction::Right => x += 1,
                    Direction::Down  => y -= 1,
                    Direction::Up    => y += 1,
                };
                taken_steps +=1;

                let next_cell = get_cell(x, y, self.center.x, self.center.y);

                if self.cells[next_cell.x][next_cell.y] != 0 {
                    println!("Key: x={}, y={} | Val: {}", next_cell.x, next_cell.y, taken_steps);
                }
                
                // Mark cell.
                self.cells[next_cell.x][next_cell.y] |= marker;
                // Save cost to get to the current position.
                position_to_num_of_steps.entry(next_cell).or_insert(taken_steps);
            }
        }

        self.position_to_num_of_steps_list.push((marker, position_to_num_of_steps));
    }

    fn calculate_intersections(&self, marker: u8) -> Vec<Cell> {
        let mut intersections = Vec::new();
        for x in 0..self.cells.len() {
            for y in 0..self.cells[0].len() {
                if self.cells[x][y] == marker {
                    intersections.push(Cell{x, y});
                }
            }
        }
        intersections
    }

    fn calculate_minimum_steps_to_reach_any_intersection(&self, marker: u8) -> u64 {
        let mut accumulated_marker_for_all_lines: u8 = 0;

        for (line_marker, _) in self.position_to_num_of_steps_list.iter() {
            accumulated_marker_for_all_lines |= line_marker;
        }
        if accumulated_marker_for_all_lines != marker {
            panic!("The given marker {} must be the sum of the markers of all lines (excepted {}).", marker, accumulated_marker_for_all_lines);
        }

        for (_, map) in self.position_to_num_of_steps_list.iter() {
            // for (&key, &value) in map.iter() {
            //     println!("Key: x={}, y={}, val={}", key.x, key.y, value);
            // }
            panic!("Waaaay");
        }


        let mut steps_to_reach_intersections: Vec<u64> = Vec::new();

        let intersections = self.calculate_intersections(marker);
        for intersection in intersections.iter() {
            let mut taken_steps: u64 = 0;

            for (_, position_to_num_of_steps) in self.position_to_num_of_steps_list.iter() {
                if position_to_num_of_steps.get(intersection) == None {
                    println!("Did not find x = {}, y = {}", intersection.x, intersection.y);
                    continue;
                }

                taken_steps += position_to_num_of_steps.get(intersection).unwrap();
            }

            steps_to_reach_intersections.push(taken_steps);
        }
        println!("XXX: intersections: {}, steps_to_reach_intersections: {}", intersections.len(), steps_to_reach_intersections.len());
        steps_to_reach_intersections.sort();
        *steps_to_reach_intersections.first().unwrap()
    }
}

fn main() {
    let line_a = vec!["R1009","U286","L371","U985","R372","D887","R311","U609","L180","D986","L901","D592","R298","U955","R681","D68","R453","U654","L898","U498","R365","D863","L974","U333","L267","D230","R706","D67","L814","D280","R931","D539","R217","U384","L314","D162","L280","U484","L915","D512","L974","D220","R292","U465","L976","U837","R28","U68","L98","D177","L780","U732","R696","D412","L715","U993","L617","U999","R304","D277","R889","D604","R199","U498","R302","U958","R443","U957","R453","U362","R704","U301","R813","U404","L150","D673","L407","D233","L901","D965","R602","U615","R496","U467","R849","U530","L205","D43","R709","U127","L35","U801","L565","D890","R90","D763","R95","D542","R84","D421","L298","D58","R794","U722","R205","U830","L149","D759","L950","D708","L727","U401","L187","D598","L390","D469","R375","U985","L723","U63","L983","D39","L160","U276","R822","D504","L298","D484","L425","U228","L984","D623","L936","U624","L851","D748","L266","D576","L898","U783","L374","D276","R757","U89","L649","U73","L447","D11","L539","U291","L507","U208","R167","D874","L596","D235","R334","U328","R41","D212","L544","D72","L972","D790","L282","U662","R452","U892","L830","D86","L252","U701","L215","U179","L480","U963","L897","U489","R223","U757","R804","U373","R844","D518","R145","U304","L24","D988","R605","D644","R415","U34","L889","D827","R854","U836","R837","D334","L664","D883","L900","U448","R152","U473","R243","D147","L711","U642","R757","U272","R192","U741","L522","U785","L872","D128","L161","D347","L967","D295","R831","U535","R329","D752","R720","D806","R897","D320","R391","D737","L719","U652","L54","D271","L855","D112","R382","U959","R909","D687","L699","U892","L96","D537","L365","D182","R886","U566","R929","U532","L255","U823","R833","U542","R234","D339","R409","U100","L466","U572","L162","U843","L635","D153","L704","D317","L534","U205","R611","D672","L462","D506","L243","U509","L819","D787","R448","D353","R162","U108","R850","D919","R259","U877","R50","D733","L875","U106","L890","D275","L904","U849","L855","U314","L291","U170","L627","U608","R783","U404","R294"];
    let line_b = vec!["L1010","D347","R554","U465","L30","D816","R891","D778","R184","U253","R694","U346","L743","D298","L956","U703","R528","D16","L404","D818","L640","D50","R534","D99","L555","U974","L779","D774","L690","U19","R973","D588","L631","U35","L410","D332","L74","D858","R213","U889","R977","U803","L624","U627","R601","U499","L213","U692","L234","U401","L894","U733","R414","D431","R712","D284","R965","D624","R848","D17","R86","D285","R502","U516","L709","U343","L558","D615","L150","D590","R113","D887","R469","U584","L434","D9","L994","D704","R740","D541","R95","U219","L634","D184","R714","U81","L426","D437","R927","U232","L361","D756","R685","D206","R116","U844","R807","U811","L382","D338","L660","D997","L551","D294","L895","D208","R37","D90","R44","D131","R77","U883","R449","D24","R441","U659","R826","U259","R98","D548","R118","D470","L259","U170","R518","U731","L287","U191","L45","D672","L691","U117","R156","U308","R230","U112","L938","U644","R911","U110","L1","U162","R943","U433","R98","U610","R428","U231","R35","U590","R554","U612","R191","U261","R793","U3","R507","U632","L571","D535","R30","U281","L613","U199","R168","D948","R486","U913","R534","U131","R974","U399","L525","D174","L595","D567","L394","D969","L779","U346","L969","D943","L845","D727","R128","U241","L616","U117","R791","D419","L913","D949","R628","D738","R776","D294","L175","D708","R568","U484","R589","D930","L416","D114","L823","U16","R260","U450","R534","D94","R695","D982","R186","D422","L789","D886","L761","U30","R182","U930","L483","U863","L318","U343","L380","U650","R542","U92","L339","D390","L55","U343","L641","D556","R616","U936","R118","D997","R936","D979","L594","U326","L975","U52","L89","U679","L91","D969","R878","D798","R193","D858","R95","D989","R389","U960","R106","D564","R48","D151","L121","D241","L369","D476","L24","D229","R601","U849","L632","U894","R27","U200","L698","U788","L330","D73","R405","D526","L154","U942","L504","D579","L815","D643","L81","U172","R879","U28","R715","U367","L366","D964","R16","D415","L501","D176","R641","U523","L979","D556","R831"];
    
    enum FieldValues {
        Empty = 0,
        Center = 1,
        PointA = 2,
        PointB = 4,
    };

    let mut wire_field = Field::new(
        100, 
        FieldValues::Empty as u8, 
        FieldValues::Center as u8);

    wire_field.add_line(&line_a, FieldValues::PointA as u8);
    wire_field.add_line(&line_b, FieldValues::PointB as u8);

    let intersections = wire_field.calculate_intersections(FieldValues::PointA as u8 | FieldValues::PointB as u8);
    if intersections.is_empty() {
        println!("No intersections found.");
        return
    }

    let mut distances: Vec<i64> = intersections.iter()
        .map(|cell| (cell.x as i64 - wire_field.center.x as i64).abs() + (cell.y as i64 - wire_field.center.y as i64).abs())
        .collect();
    
    for i in 0..intersections.len() {
        println!("Found x = {}, y = {}, distance = {}.", intersections[i].x, intersections[i].y, distances[i]);
    }

    distances.sort();
    println!("Shortest distance of all intersections: {}.", distances.first().unwrap());

    // Todo remove
    println!("Center: x={}, y={}", wire_field.center.x, wire_field.center.y);

    let minimum_steps_to_reach_any_intersection = 
        wire_field.calculate_minimum_steps_to_reach_any_intersection(FieldValues::PointA as u8 | FieldValues::PointB as u8);
    println!("Minimum wire distance to reach an intersection: {}.", minimum_steps_to_reach_any_intersection);
}
