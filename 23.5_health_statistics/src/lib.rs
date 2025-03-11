/*
You’re working on implementing a health-monitoring system.
As part of that, you need to keep track of users’ health statistics.

You’ll start with a stubbed function in an impl block as well as a User struct definition.
Your goal is to implement the stubbed out method on the User struct defined in the impl block.
*/

#![allow(dead_code)]
pub struct User {
	name: String,
	age: u32,
	height: f32,
	visit_count: u32,
	last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
	height: f32,
	blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
	patient_name: &'a str,
	visit_count: u32,
	height_change: f32,
	blood_pressure_change: Option<(i32, i32)>,
}

impl User {
	pub fn new(name: String, age: u32, height: f32) -> Self {
		Self {
			name,
			age,
			height,
			visit_count: 0,
			last_blood_pressure: None,
		}
	}

	pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
		// update height and blood_pressure on user, changes into health report
		let height_diff = (self.height - measurements.height).abs();
		let (new_systolic, new_diastolic) = measurements.blood_pressure;
		let bp_diff: (i32, i32) = if self.last_blood_pressure.is_some() {
			let (last_systolic, last_diastolic) = self.last_blood_pressure.unwrap();
			(
				(0 - last_systolic as i32 + new_systolic as i32),
				(0 - last_diastolic as i32 + new_diastolic as i32),
			)
		} else {
			(0, 0)
		};
		self.height = measurements.height;
		self.last_blood_pressure = Some(measurements.blood_pressure);
		self.visit_count += 1;

		HealthReport {
			patient_name: &self.name,
			visit_count: self.visit_count,
			height_change: height_diff,
			blood_pressure_change: if bp_diff == (0, 0) {
				None
			} else {
				Some(bp_diff)
			},
		}
		// todo!("Update a user's statistics based on measurements from a visit to the doctor")
	}
}

#[test]
fn test_visit() {
	let mut bob = User::new(String::from("Bob"), 32, 155.2);
	assert_eq!(bob.visit_count, 0);
	let report = bob.visit_doctor(Measurements {
		height: 156.1,
		blood_pressure: (120, 80),
	});
	assert_eq!(report.patient_name, "Bob");
	assert_eq!(report.visit_count, 1);
	assert_eq!(report.blood_pressure_change, None);
	assert!((report.height_change - 0.9).abs() < 0.00001);

	let report = bob.visit_doctor(Measurements {
		height: 156.1,
		blood_pressure: (115, 76),
	});

	assert_eq!(report.visit_count, 2);
	assert_eq!(report.blood_pressure_change, Some((-5, -4)));
	assert_eq!(report.height_change, 0.0);
}
