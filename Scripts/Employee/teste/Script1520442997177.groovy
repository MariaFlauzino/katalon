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
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://opensource.demo.orangehrmlive.com/')

WebUI.setText(findTestObject('Page_Employee/teste/input_txtUsername'), 'admin')

WebUI.setText(findTestObject('Page_Employee/teste/input_txtPassword'), 'admin')

WebUI.sendKeys(findTestObject('Page_Employee/teste/input_txtPassword'), Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Page_Employee/teste/b_PIM'))

WebUI.click(findTestObject('Page_Employee/teste/a_Add Employee'))

WebUI.click(findTestObject('Page_Employee/input_btnSave'))

CustomKeywords.'util.SelectFile.SelectFile'(findTestObject('Page_Employee/teste/input_photofile'), 'C:\\Users\\saomflau\\Desktop\\Teste\\katalon\\Data Files\\Images', 
    'Firefox.PNG')

