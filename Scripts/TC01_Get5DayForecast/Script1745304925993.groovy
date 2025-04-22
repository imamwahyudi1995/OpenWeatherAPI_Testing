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
import com.kms.katalon.core.util.KeywordUtil
import groovy.json.JsonSlurper


import static org.assertj.core.api.Assertions.*

// Kirim request ke endpoint 5-day forecast
def response = WS.sendRequest(findTestObject('Get_WeatherForecast'))

// Validasi status code
WS.verifyResponseStatusCode(response, 200)

// Parse response JSON
def json = new JsonSlurper().parseText(response.getResponseText())

// Validasi struktur dasar JSON
assertThat(json.cod).isEqualTo("200")
assertThat(json).containsKeys("list", "city")

// Validasi isi dari 'list'
assertThat(json.list).isNotEmpty()
json.list.each { item ->
	assertThat(item).containsKeys("dt", "main", "weather", "dt_txt")
	assertThat(item.main).containsKey("temp")
	assertThat(item.weather[0]).containsKeys("main", "description")
}
