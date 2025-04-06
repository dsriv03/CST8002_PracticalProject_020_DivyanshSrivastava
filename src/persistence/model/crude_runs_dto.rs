/*
    Author: Divyansh Srivastava
    Student Number: 041109063
    Submission: Practical Project for CST 8002_020
    Submission Date: 
*/


use chrono::NaiveDate;
use tabled::Tabled;

/// Struct CrudeRunsDTO meant as Data Transfer Object for the crude runs csv
#[derive(serde::Serialize)]
#[derive(PartialEq)]
#[derive(Debug, Clone)]
#[derive(Tabled)]
pub struct CrudeRunsDTO {
    
    /// 64 bit signed int, ID of entry
    id: i64,

    /// Date without timezone in MM/DD/YYYY, January 1990 to most recently available
    week_end: chrono::NaiveDate,

    /// Date without timezone in MM/DD/YYYY, corresponding week end last year
    week_end_last_year: chrono::NaiveDate,

    /// Ontario, Quebec & Eastern Canada, Western Canada
    region: String,

    ///Volume of crude oil processed by refineries
    volume: f64,

    /// Volume of crude oil processed as percent of refinery capacity
    capacity: f64,

    /// Crude runs average for the last 4 weeks
    four_week_avg: f64,

    /// Crude runs average for the correspomding 4 weeks last year
    four_week_avg_lastyear: f64,

    /// Average crude runs year to date
    ytd_avg: f64,

    /// Average crude runs year to date last year
    ytd_avg_lastyear: f64,

    /// Unit of measurement for volumetric measurements
    unit: String

}

/// Default crude run struct, in the offchance an entry fails to import, a default entry will be imported instead with the corresponding ID in the csv for tracking purposes
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

/// Implements Struct specific methods: Setters (No getters as they're unnecessary for now), and string representation of struct
impl CrudeRunsDTO {

    /// Sets weekly end date
    ///
    /// # Arguments
    ///
    /// * `date`
    ///
    pub fn set_weekly_end(&mut self, date: NaiveDate){
        self.week_end = date;
    }

    /// Sets weekly end date for the last year
    ///
    /// # Arguments
    ///
    /// * `date`
    ///
    pub fn set_week_end_last_year(&mut self, date: NaiveDate){
        self.week_end_last_year = date;
    }

    /// Sets region
    ///
    /// # Arguments
    ///
    /// * `region`
    ///
    pub fn set_region(&mut self, region: String){
        self.region = region;
    }

    /// Sets volume
    ///
    /// # Arguments
    ///
    /// * `volume`
    ///
    pub fn set_volume(&mut self, volume: f64){
        self.volume = volume;
    }

    /// sets capacity
    ///
    /// # Arguments
    ///
    /// * `capacity`
    ///
    pub fn set_capacity(&mut self, capacity: f64){
        self.capacity = capacity;
    }

    /// Sets 4-week average volume
    ///
    /// # Arguments
    ///
    /// * `average volume`
    ///
    pub fn set_four_week_avg(&mut self, avg: f64){
        self.four_week_avg = avg;
    }

    /// Sets 4-week average volume for last year
    ///
    /// # Arguments
    ///
    /// * `average volume`
    ///
    pub fn set_four_week_avg_lastyear(&mut self, avg: f64){
        self.four_week_avg_lastyear = avg;
    }

    /// Sets year to date average
    ///
    /// # Arguments
    ///
    /// * `average`
    ///
    pub fn set_ytd_avg(&mut self, avg: f64){
        self.ytd_avg = avg;
    }

    /// Sets year to date average for last year
    ///
    /// # Arguments
    ///
    /// * `average`
    ///
    pub fn set_ytd_avg_last_year(&mut self, avg: f64){
        self.ytd_avg_lastyear = avg;
    }

    /// Sets unit of measurement for volumes
    ///
    /// # Arguments
    ///
    /// * `unit`
    ///
    pub fn set_unit(&mut self, unit: String){
        self.unit = unit;
    }

    /// Sets ID for each crude run object (Useful for transferring them into a database)
    ///
    /// # Arguments
    ///
    /// * `ID`
    ///
    pub fn set_id(&mut self, id: i64){
        self.id = id;
    }

    // Getter for `id`
    pub fn get_id(&self) -> i64 {
        self.id
    }

    // Getter for `week_end`
    pub fn get_week_end(&self) -> &NaiveDate {
        &self.week_end
    }

    // Getter for `week_end_last_year`
    pub fn get_week_end_last_year(&self) -> &NaiveDate {
        &self.week_end_last_year
    }

    // Getter for `region`
    pub fn get_region(&self) -> &str {
        &self.region
    }

    // Getter for `volume`
    pub fn get_volume(&self) -> f64 {
        self.volume
    }

    // Getter for `capacity`
    pub fn get_capacity(&self) -> f64 {
        self.capacity
    }

    // Getter for `four_week_avg`
    pub fn get_four_week_avg(&self) -> f64 {
        self.four_week_avg
    }

    // Getter for `four_week_avg_lastyear`
    pub fn get_four_week_avg_lastyear(&self) -> f64 {
        self.four_week_avg_lastyear
    }

    // Getter for `ytd_avg`
    pub fn get_ytd_avg(&self) -> f64 {
        self.ytd_avg
    }

    // Getter for `ytd_avg_lastyear`
    pub fn get_ytd_avg_lastyear(&self) -> f64 {
        self.ytd_avg_lastyear
    }

    // Getter for `unit`
    pub fn get_unit(&self) -> &str {
        &self.unit
    }

    /// Prints each field in the crude run DTO struct
    pub fn to_string(&self){
        println!("id: {},
        week_end: {},
        week_end_last_year: {},
        region: {},
        volume: {},
        capacity: {},
        four_week_avg: {},
        four_week_avg_lastyear: {},
        ytd_avg: {},
        ytd_avg_lastyear: {},
        unit: {}",
        self.id,
        self.week_end, self.week_end_last_year,
        self.region, self.volume,
        self.capacity, self.four_week_avg,
        self.four_week_avg_lastyear, self.ytd_avg,
        self.ytd_avg_lastyear, self.unit)
    }
}