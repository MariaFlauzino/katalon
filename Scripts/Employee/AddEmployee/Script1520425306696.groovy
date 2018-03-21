import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.callTestCase(findTestCase('General/Login'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Page_Employee/b_PIM'))

WebUI.click(findTestObject('Page_Employee/a_Add Employee'))

WebUI.sendKeys(findTestObject('Page_Employee/input_firstName'), FirstName)

WebUI.sendKeys(findTestObject('Page_Employee/input_middleName'), MiddleName)

WebUI.sendKeys(findTestObject('Page_Employee/input_lastName'), LastName)

CustomKeywords.'util.SelectFile.SelectFile'(findTestObject('Page_Employee/input_photofile'), '\\Data Files\\Images', 'Firefox.PNG')

if (CreateLoginDetails == 'yes') {
    WebUI.check(findTestObject('Page_Employee/input_chkLogin'))

    WebUI.sendKeys(findTestObject('Page_Employee/input_user_name'), UserName)

    WebUI.setText(findTestObject('Page_Employee/input_user_password'), Password)

    WebUI.setText(findTestObject('Page_Employee/input_re_password'), ConfirmPassword)

    WebUI.selectOptionByValue(findTestObject('Page_Employee/select_EnabledDisabled'), Status, false)
}

WebUI.click(findTestObject('Page_Employee/input_btnSave'))

WebUI.verifyElementVisible(findTestObject('Page_Employee/h1_Personal Details'))

WebUI.callTestCase(findTestCase('General/Logout and Close'), [:], FailureHandling.CONTINUE_ON_FAILURE)

