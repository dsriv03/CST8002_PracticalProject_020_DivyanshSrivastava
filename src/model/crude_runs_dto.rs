use chrono::NaiveDate;

pub struct CrudeRunsDTO {
    
    id: u128,
    week_end: chrono::NaiveDate,
    week_end_last_year: chrono::NaiveDate,
    region: String,
    volume: f64,
    capacity: f64,
    four_week_avg: f64,
    four_week_avg_lastyear: f64,
    ytd_avg: f64,
    ytd_avg_lastyear: f64,
    unit: String

}

impl Default for CrudeRunsDTO{
    fn default() -> Self {
        Self {
            //hard coded default values possibly bad design, TODO: find better invalid field handling logic
            id: 0,
            week_end: NaiveDate::MIN,
            week_end_last_year: NaiveDate::MIN,
            //TODO: assign better default values
            region: Default::default(),
            volume: Default::default(), 
            capacity: Default::default(),
            four_week_avg: Default::default(),
            four_week_avg_lastyear: Default::default(),
            ytd_avg: Default::default(),
            ytd_avg_lastyear: Default::default(),
            unit: Default::default() }
    }
}

impl CrudeRunsDTO {

    pub fn set_weekly_end(&mut self, date: NaiveDate){
        self.week_end = date;
    }

    pub fn set_week_end_last_year(&mut self, date: NaiveDate){
        self.week_end_last_year = date;
    }

    pub fn set_region(&mut self, region: String){
        self.region = region;
    }

    pub fn set_volume(&mut self, volume: f64){
        self.volume = volume;
    }

    pub fn set_capacity(&mut self, capacity: f64){
        self.capacity = capacity;
    }

    pub fn set_four_week_avg(&mut self, avg: f64){
        self.four_week_avg = avg;
    }

    pub fn set_four_week_avg_lastyear(&mut self, avg: f64){
        self.four_week_avg_lastyear = avg;
    }

    pub fn set_ytd_avg(&mut self, avg: f64){
        self.ytd_avg = avg;
    }

    pub fn set_ytd_avg_last_year(&mut self, avg: f64){
        self.ytd_avg_lastyear = avg;
    }

    pub fn set_unit(&mut self, unit: String){
        self.unit = unit;
    }

    pub fn set_id(&mut self, id: u128){
        self.id = id;
    }

    pub fn to_string(&mut self){
        println!("id: {},
        week_end: {},
        week_end_last_year: {},
        region: {},
        volume: {},
        capacity: {},
        four_week_avg: {},
        four_week_avg_lastyear: {},
        ytd_avg: {},
        ytd_avg_lastyear{},
        unit: {}",
        self.id,
        self.week_end, self.week_end_last_year,
        self.region, self.volume,
        self.capacity, self.four_week_avg,
        self.four_week_avg_lastyear, self.ytd_avg,
        self.ytd_avg_lastyear, self.unit)
    }
}