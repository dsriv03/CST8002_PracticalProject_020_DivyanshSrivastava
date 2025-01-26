use time;

pub struct CrudeRunsDTO {
    
    week_end: time::Date, //a
    week_end_last_year: time::Date, //a
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
        //unhandled panic if input out of range (hard-coded so not possible, still not an elegent solution)
        let default_time = time::Date::from_calendar_date(1970, time::Month::January, 1).unwrap();
        Self { week_end: default_time,
            week_end_last_year: default_time,
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

    pub fn set_weekly_end(&mut self, date: time::Date){
        self.week_end = date;
    }

    pub fn set_week_end_last_year(&mut self, date: time::Date){
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

}