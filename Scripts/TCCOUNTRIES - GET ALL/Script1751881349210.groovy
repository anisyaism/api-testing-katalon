import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper as JsonSlurper

response = WS.sendRequest(findTestObject('Countries/all', [('BASE_URL_COUNTRIES') : GlobalVariable.BASE_URL_COUNTRIES]))

WS.verifyResponseStatusCode(response, 200)

// Step 3: Parse JSON response
def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

// Step 4: Basic assertion – make sure we got at least one country
assert jsonResponse.size() > 0 : 'There are a country'

// Step 5: Loop through and print all country names
println('==== Country List ====')

jsonResponse.eachWithIndex({ def country, def index ->
        println("$(index + 1). $country.name.common")
    })

// Step 6: Specific checks (example: first country should be Indonesia)
def firstCountry = jsonResponse[0]

WS.verifyElementPropertyValue(response, '[0].name.common', 'Togo')

assert firstCountry.name.official == 'Togolese Republic'

assert firstCountry.name.nativeName.fra.official == 'République togolaise'

