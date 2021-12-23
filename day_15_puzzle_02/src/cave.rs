use std::cmp;

pub struct CaveBuilder {
	width: Option<usize>,
	source_risk_levels: Vec<Vec<u32>>,
}

impl CaveBuilder {
	pub fn new() -> CaveBuilder {
		CaveBuilder {
			width: None,
			source_risk_levels: vec![],
		}
	}

	pub fn add_row(&mut self, row: Vec<u32>) {
		if let Some(width) = self.width {
			if width != row.len() {
				panic!("Row does not match the expected length of {} - found {}", width, row.len());
			}
		} else {
			self.width = Some(row.len());
		}

		self.source_risk_levels.push(row);
	}

	pub fn build_full(self) -> Cave {
		let mut full_cave = vec![];
		let source_height = self.source_risk_levels.len();
		let source_width = self.source_risk_levels[0].len();
		let full_height = source_height * 5;
		let full_width = source_width * 5;
		let find_source_risk = |x, y| -> u32 {
			let source_x: usize = x % source_width;
			let source_y: usize = y % source_height;

			self.source_risk_levels[source_y][source_x]
		};
		let transform_risk = |x, y| -> u32 {
			let x_offset = x / source_width;
			let y_offset = y / source_height;

			let source_risk = find_source_risk(x, y);
			let transformed_risk = source_risk + x_offset as u32 + y_offset as u32;
			if transformed_risk < 10 {
				transformed_risk
			} else {
				transformed_risk - 9
			}
		};
		for y in 0..full_height {
			let mut full_row = vec![];
			for x in 0..full_width {
				let new_risk = transform_risk(x, y);
				full_row.push(new_risk);
			}
			full_cave.push(full_row);
		}
		Cave::new(full_cave)
	}
}

pub struct Cave {
	risk_levels: Vec<Position>,
	width: usize,
	height: usize,
}

impl Cave {
	pub fn new(risk_grid: Vec<Vec<u32>>) -> Cave {
		let mut risk_levels = vec![];
		for (y, row) in risk_grid.iter().enumerate() {
			for (x, risk) in row.iter().enumerate() {
				risk_levels.push(Position::new(x, y, *risk));
			}
		}
		Cave {
			risk_levels,
			width: risk_grid[0].len(),
			height: risk_grid.len(),
		}
	}

	pub fn start(&self) -> &Position {
		self.get_pos(0, 0)
	}

	pub fn end(&self) -> &Position {
		self.get_pos(self.width - 1, self.height - 1)
	}

	fn get_pos(&self, x: usize, y: usize) -> &Position {
		let index = x + y * self.width;
		&self.risk_levels[index]
	}

	pub fn neighbors(&self, pos: &Position) -> Vec<&Position> {
		let mut neighbors = vec![];

		if pos.x > 0 {
			neighbors.push(self.get_pos(pos.x - 1, pos.y));
		}
		if pos.x < self.width - 1 {
			neighbors.push(self.get_pos(pos.x + 1, pos.y));
		}
		if pos.y > 0 {
			neighbors.push(self.get_pos(pos.x, pos.y - 1));
		}
		if pos.y < self.height - 1 {
			neighbors.push(self.get_pos(pos.x, pos.y + 1));
		}

		neighbors
	}
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Position {
	x: usize,
	y: usize,
	risk: u32,
}

impl Position {
	pub fn new(x: usize, y: usize, risk: u32) -> Position {
		Position {
			x,
			y,
			risk,
		}
	}

	pub fn risk(&self) -> u32 {
		self.risk
	}

	pub fn distance(&self, other: &Self) -> usize {
		(cmp::max(self.x, other.x) - cmp::min(self.x, other.x)) + (cmp::max(self.y, other.y) - cmp::min(self.y, other.y))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	impl CaveBuilder {
		pub fn build(self) -> Cave {
			Cave::new(self.source_risk_levels)
		}
	}

	#[test]
	fn test_build_full() {
		let source_rows = "
		1163751742
		1381373672
		2136511328
		3694931569
		7463417111
		1319128137
		1359912421
		3125421639
		1293138521
		2311944581"
			.trim()
            .split("\n")
            .map(|r| r.trim().chars()
                .map(|c| u32::from_str_radix(&c.to_string(), 10).unwrap())
                .collect::<Vec<u32>>()
            ).collect::<Vec<Vec<u32>>>();

		let mut builder = CaveBuilder::new();
		for row in source_rows.into_iter() {
			builder.add_row(row);
		}

		let built_cave = builder.build_full();

		let full_rows = "
		11637517422274862853338597396444961841755517295286
		13813736722492484783351359589446246169155735727126
		21365113283247622439435873354154698446526571955763
		36949315694715142671582625378269373648937148475914
		74634171118574528222968563933317967414442817852555
		13191281372421239248353234135946434524615754563572
		13599124212461123532357223464346833457545794456865
		31254216394236532741534764385264587549637569865174
		12931385212314249632342535174345364628545647573965
		23119445813422155692453326671356443778246755488935
		22748628533385973964449618417555172952866628316397
		24924847833513595894462461691557357271266846838237
		32476224394358733541546984465265719557637682166874
		47151426715826253782693736489371484759148259586125
		85745282229685639333179674144428178525553928963666
		24212392483532341359464345246157545635726865674683
		24611235323572234643468334575457944568656815567976
		42365327415347643852645875496375698651748671976285
		23142496323425351743453646285456475739656758684176
		34221556924533266713564437782467554889357866599146
		33859739644496184175551729528666283163977739427418
		35135958944624616915573572712668468382377957949348
		43587335415469844652657195576376821668748793277985
		58262537826937364893714847591482595861259361697236
		96856393331796741444281785255539289636664139174777
		35323413594643452461575456357268656746837976785794
		35722346434683345754579445686568155679767926678187
		53476438526458754963756986517486719762859782187396
		34253517434536462854564757396567586841767869795287
		45332667135644377824675548893578665991468977611257
		44961841755517295286662831639777394274188841538529
		46246169155735727126684683823779579493488168151459
		54698446526571955763768216687487932779859814388196
		69373648937148475914825958612593616972361472718347
		17967414442817852555392896366641391747775241285888
		46434524615754563572686567468379767857948187896815
		46833457545794456865681556797679266781878137789298
		64587549637569865174867197628597821873961893298417
		45364628545647573965675868417678697952878971816398
		56443778246755488935786659914689776112579188722368
		55172952866628316397773942741888415385299952649631
		57357271266846838237795794934881681514599279262561
		65719557637682166874879327798598143881961925499217
		71484759148259586125936169723614727183472583829458
		28178525553928963666413917477752412858886352396999
		57545635726865674683797678579481878968159298917926
		57944568656815567976792667818781377892989248891319
		75698651748671976285978218739618932984172914319528
		56475739656758684176786979528789718163989182927419
		67554889357866599146897761125791887223681299833479"
			.trim()
            .split("\n")
            .map(|r| r.trim().chars()
                .map(|c| u32::from_str_radix(&c.to_string(), 10).unwrap())
                .collect::<Vec<u32>>()
            ).collect::<Vec<Vec<u32>>>();
		
		let mut builder = CaveBuilder::new();
		for row in full_rows.into_iter() {
			builder.add_row(row);
		}

		let full_cave = builder.build();

		for x in 0..full_cave.width {
			for y in 0..full_cave.height {
				let full_pos = full_cave.get_pos(x, y);
				let built_pos = built_cave.get_pos(x, y);
				assert_eq!(full_pos, built_pos);
			}
		}
	}
}