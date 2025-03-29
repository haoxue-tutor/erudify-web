use anyhow::{ensure, Context, Result};
use reqwest::StatusCode;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio::time::sleep;

async fn wait_for_service(url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    for _ in 0..60 {
        match client.get(url).send().await {
            Ok(response) if response.status() == StatusCode::OK => return Ok(()),
            _ => {
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
    anyhow::bail!("Service did not become ready within 60 seconds")
}

async fn wait_for_webdriver(url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    for _ in 0..60 {
        match client.get(url).send().await {
            Ok(_) => return Ok(()),
            _ => {
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
    anyhow::bail!("WebDriver did not become ready within 60 seconds")
}

#[tokio::main]
async fn main() -> Result<()> {
    // Wait for the web service to be ready
    wait_for_service("http://localhost:8787")
        .await
        .context("Failed waiting for web service")?;

    // Wait for ChromeDriver to be ready
    wait_for_webdriver("http://localhost:4444")
        .await
        .context("Failed waiting for ChromeDriver")?;

    // Connect to WebDriver instance
    let mut caps = DesiredCapabilities::firefox();
    // let mut caps = DesiredCapabilities::safari();
    caps.set_headless()?;
    let driver = WebDriver::new("http://localhost:4444", caps)
        .await
        .context("Failed to connect to WebDriver")?;

    let ret = tests(&driver).await;

    // Clean up
    driver
        .quit()
        .await
        .context("Failed to quit WebDriver session")?;

    match ret {
        Ok(_) => {
            println!("E2E test passed successfully!");
            Ok(())
        }
        Err(e) => {
            println!("E2E test failed!");
            Err(e)
        }
    }
}

async fn tests(driver: &WebDriver) -> Result<()> {
    check_title(driver, "Erudify").await?;
    check_stylesheet_loaded(driver).await?;
    Ok(())
}

async fn check_title(driver: &WebDriver, expected_title: &str) -> Result<()> {
    // Navigate to the website
    driver
        .goto("http://localhost:8787")
        .await
        .context("Failed to navigate to website")?;

    // Get the page title
    let title = driver.title().await.context("Failed to get page title")?;

    // Check if title contains expected text
    ensure!(
        title.contains(expected_title),
        "Page title '{}' does not contain '{}'",
        title,
        expected_title
    );
    Ok(())
}

async fn check_stylesheet_loaded(driver: &WebDriver) -> Result<()> {
    // Wait for any element that should have CSS applied
    // For example, let's check the body element
    let body = driver
        .find(By::Tag("body"))
        .await
        .context("Failed to find body element")?;

    // Get the computed style of the element
    let background_color = driver
        .execute(
            "return window.getComputedStyle(arguments[0]).backgroundColor",
            vec![body.to_json()?],
        )
        .await
        .context("Failed to get computed style")?;
    let background_color = background_color.json().as_str().unwrap_or("").to_string();

    // Verify that the background color is not undefined or empty
    ensure!(
        !background_color.is_empty(),
        "CSS styles not loaded - background-color is empty"
    );

    Ok(())
}
