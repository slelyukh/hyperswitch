use serial_test::serial;
use thirtyfour::{prelude::*, WebDriver};

use crate::{selenium::*, tester};

struct AirwallexSeleniumTest;

impl SeleniumTest for AirwallexSeleniumTest {
    fn get_connector_name(&self) -> String {
        "airwallex".to_string()
    }
}

async fn should_make_airwallex_3ds_payment(c: WebDriver) -> Result<(), WebDriverError> {
    let conn = AirwallexSeleniumTest {};
    conn.make_redirection_payment(c, vec![
            Event::Trigger(Trigger::Goto(&format!("{CHEKOUT_BASE_URL}/card?cname=CL-BRW1&ccnum=4012000300000088&expmonth=10&expyear=2025&cvv=123&amount=2000&country=US&currency=GBP"))),
            Event::Assert(Assert::IsPresent("Expiry Year")),
            Event::Trigger(Trigger::Click(By::Id("card-submit-btn"))),
            Event::Trigger(Trigger::Query(By::ClassName("title"))),
            Event::Assert(Assert::Eq(Selector::Title, "Airwallex - Create 3D Secure Payment")),
            Event::Trigger(Trigger::SendKeys(By::Id("challengeDataEntry"), "1234")),
            Event::Trigger(Trigger::Click(By::Id("submit"))),
            Event::Assert(Assert::IsPresent("Google")),
            Event::Assert(Assert::Contains(Selector::QueryParamStr, "status=succeeded")),

    ]).await?;
    Ok(())
}

#[test]
#[serial]
fn should_make_airwallex_3ds_payment_test() {
    tester!(should_make_airwallex_3ds_payment);
}