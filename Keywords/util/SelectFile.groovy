package util

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords
import com.sun.java.swing.plaf.windows.resources.windows

import internal.GlobalVariable
import sun.util.locale.provider.LocaleResources
import MobileBuiltInKeywords as Mobile
import WSBuiltInKeywords as WS
import WebUiBuiltInKeywords as WebUI

import java.awt.EventQueue
import java.awt.Robot
import java.awt.Toolkit
import java.awt.Window
import java.awt.datatransfer.StringSelection
import java.awt.event.KeyEvent
import java.awt.event.WindowFocusListener

import org.openqa.selenium.WebDriver
import org.openqa.selenium.support.ui.Wait

import java.awt.event.ComponentEvent
import java.awt.event.*;

@Keyword
def SelectFile(TestObject to, String relativePath, String nameFile) {
	Robot robot = new Robot();
	robot.setAutoDelay(500);

	WebUI.click(to);
	setPathFile (robot, relativePath);
	pastValue(robot);
	WebUI.waitForAlert(1);
	setNameFile(robot, nameFile);
	pastValue(robot);
}

def setPathFile (Robot robot, String relativePath){
	//muda o foco para o caminho do arquivo

	StringSelection ss = new StringSelection(fullPath(relativePath));
	
	if(languageOS() == 'en_US'){
		robot.keyPress(KeyEvent.VK_ALT);
		robot.keyPress(KeyEvent.VK_D);
		robot.keyRelease(KeyEvent.VK_ALT);
		robot.keyRelease(KeyEvent.VK_D);
	}else if (languageOS() == 'pt_BR'){
		robot.keyPress(KeyEvent.VK_ALT);
		robot.keyPress(KeyEvent.VK_D);
		robot.keyRelease(KeyEvent.VK_ALT);
		robot.keyRelease(KeyEvent.VK_D);
	}
	
	Toolkit.getDefaultToolkit().getSystemClipboard().setContents(ss, null);
}

def setNameFile(Robot robot, String nameFile){
	//muda o foco para o nome do arquivo
	StringSelection nf = new StringSelection(nameFile);
	robot.keyPress(KeyEvent.VK_ALT);
	robot.keyPress(KeyEvent.VK_N);
	robot.keyRelease(KeyEvent.VK_ALT);
	robot.keyRelease(KeyEvent.VK_N);
	Toolkit.getDefaultToolkit().getSystemClipboard().setContents(nf, null);
}

def pastValue(Robot robot){
	robot.keyPress(KeyEvent.VK_CONTROL);
	robot.keyPress(KeyEvent.VK_V);
	robot.keyRelease(KeyEvent.VK_V);
	robot.keyRelease(KeyEvent.VK_CONTROL);
	robot.keyPress(KeyEvent.VK_ENTER);
}

def String fullPath (String relativePath){
	String userDir = System.getProperty("user.dir");
	String filePath = userDir + relativePath;
}
	
def String languageOS (){
	String lan = Locale.getDefault();		
}


