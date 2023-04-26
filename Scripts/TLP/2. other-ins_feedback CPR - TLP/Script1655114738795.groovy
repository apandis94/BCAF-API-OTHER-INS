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
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil

token = WS.sendRequest(findTestObject('CPR/get_credential_CPR'))

'store json response to variable'
def slurper = new groovy.json.JsonSlurper()

def get_token = slurper.parseText(token.getResponseBodyContent())

'define / field api'
def private_token = get_token.access_token

'store var to global variable'
GlobalVariable.g_token = private_token

println('token is :' + GlobalVariable.g_token)

'show response to report in test suite'
response_token = token.getResponseText()

KeywordUtil.logInfo("$response_token")

feedback = WS.sendRequest(findTestObject('CPR/other-ins-feedback_CPR', [('token') : private_token, ('id') : id, ('order_date') : order_date
                , ('source') : source, ('accno') : accno, ('nopolis') : nopolis]))


WS.verifyElementPropertyValue(feedback, 'FeedbackOrderInsCarWarrantyRs.StatusCode', '00', FailureHandling.CONTINUE_ON_FAILURE)

'show response to report in test suite'
result_feedback = feedback.getResponseText()

KeywordUtil.logInfo("$result_feedback")

feedback = WS.sendRequest(findTestObject('TLP/OtherInsFeedbackTLP', [('token') : 'owRpbUWJG0ZOKQPfLUnfb1a5b0mq', ('id') : 'TLP', ('maskapai_id') : 'TLP'
            , ('source') : 'AXA', ('orderdate') : '2022-04-19', ('no_reff') : '9430010123003', ('no_polis') : 'No1-APITLP-20220422']))
	
	WS.verifyElementPropertyValue(feedback, 'SubmitPolisAsuransiTLCResponse.StatusCode', '02', FailureHandling.CONTINUE_ON_FAILURE)
	
	'show response to report in test suite'
	result_feedback = feedback.getResponseText()
	
	KeywordUtil.logInfo("$result_feedback")



	