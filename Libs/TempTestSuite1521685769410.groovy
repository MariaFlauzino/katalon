import com.kms.katalon.core.logging.KeywordLogger
import com.kms.katalon.core.exception.StepFailedException
import com.kms.katalon.core.reporting.ReportUtil
import com.kms.katalon.core.main.TestCaseMain
import com.kms.katalon.core.testdata.TestDataColumn
import groovy.lang.MissingPropertyException
import com.kms.katalon.core.testcase.TestCaseBinding
import com.kms.katalon.core.driver.internal.DriverCleanerCollector
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.configuration.RunConfiguration
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData

Map<String, String> suiteProperties = new HashMap<String, String>();


suiteProperties.put('id', 'Test Suites/Employee/Employee')

suiteProperties.put('name', 'Employee')

suiteProperties.put('description', '')
 

DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.webui.contribution.WebUiDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.mobile.contribution.MobileDriverCleaner())



RunConfiguration.setExecutionSettingFile("C:\\Users\\Maria\\git\\katalon\\Reports\\Employee\\Employee\\20180321_232929\\execution.properties")

TestCaseMain.beforeStart()

TestCaseMain.startTestSuite('Test Suites/Employee/Employee', suiteProperties, [new TestCaseBinding('Test Cases/Employee/AddEmployeeFullWithLogin - Iteration 1', 'Test Cases/Employee/AddEmployeeFullWithLogin',  [ 'LastName' : 'Jr' , 'FirstName' : 'Caio' , 'CreateLoginDetails' : 'yes' , 'ConfirmPassword' : '123456' , 'Password' : '123456' , 'UserName' : 'Cjunior' , 'Status' : 'Enabled' , 'MiddleName' : 'Almeida' ,  ]), new TestCaseBinding('Test Cases/Employee/AddEmployeeFullWithLogin - Iteration 2', 'Test Cases/Employee/AddEmployeeFullWithLogin',  [ 'LastName' : 'Silva' , 'FirstName' : 'Bruna ' , 'CreateLoginDetails' : 'no' , 'ConfirmPassword' : '' , 'Password' : '' , 'UserName' : '' , 'Status' : '' , 'MiddleName' : 'Sampaio' ,  ])])
