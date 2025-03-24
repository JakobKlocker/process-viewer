/*
Below is example code on how to request to take a screenshot in wayland,
since this seems like its own project I will not implemnet this for this process manager for now.
Future goal would be to stream a selected process to the web. This would be a lot easyer with X11.
*/
use zbus::{Connection, Result, proxy};
use std::collections::HashMap;
use zvariant::{Value, OwnedValue, OwnedObjectPath};

#[proxy(
    interface = "org.freedesktop.portal.Screenshot",
    default_service = "org.freedesktop.portal.Desktop",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait Screenshot {
    async fn screenshot<'a>(
        &self,
        parent_window: &'a str,
        options: HashMap<&'static str, Value<'static>>,
    ) -> Result<OwnedObjectPath>;
}

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;
    let proxy = ScreenshotProxy::new(&connection).await?;

    let parent_window = "Discord";
    let mut options = HashMap::new();
    options.insert("interactive", Value::Bool(true));

    let result = proxy.screenshot(parent_window, options).await?;
    dbg!(result);
    Ok(())
}