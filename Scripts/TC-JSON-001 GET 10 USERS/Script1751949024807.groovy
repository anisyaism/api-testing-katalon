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

'Send API Request without verify it'
def responseValue = WS.sendRequest(findTestObject('JSON Placeholder/GET 10 USERS API', [('BASE_URL_JSONPlaceholder') : GlobalVariable.BASE_URL_JSONPlaceholder]))

// WS.sendRequestAndVerify(findTestObject('JSON Placeholder/GET 10 USERS API', [('BASE_URL_JSONPlaceholder') : GlobalVariable.BASE_URL_JSONPlaceholder]))
// Verify There are 10 elements
'Get how many "name" elements in the API'
def expElementCount = WS.getElementsCount(responseValue, 'name')

println(('There are = ' + expElementCount) + ' Elements')

'Verify total of "name" elements'
boolean actElementCount = WS.verifyElementsCount(responseValue, 'name', expElementCount)

if (actElementCount) {
    println('TEST PASSED')
} else {
    println('TEST FAILED')
}

// Verify There are 10 name
'Expected results of value of "name" object'
def expectedNameValue = ['Leanne Graham', 'Ervin Howell', 'Clementine Bauch', 'Patricia Lebsack', 'Chelsey Dietrich', 'Mrs. Dennis Schulist'
    , 'Kurtis Weissnat', 'Nicholas Runolfsdottir V', 'Glenna Reichert', 'Clementina DuBuque']

println('=========Name of 10 Expected Users=========')

'Print all the "name"'
for (def expUsers : expectedNameValue) {
    println(expUsers)
}

println()

println('=========Name of 10 Actual Users=========')

'Print all the Actual "name" value'
for (int i = 0; i < 10; i++) {
    WS.verifyElementPropertyValue(responseValue, "[$i].name", expectedNameValue[i])

    String usersText = WS.getElementPropertyValue(responseValue, "[$i].name")

    println('[${i + 1}].' + usersText)
}




