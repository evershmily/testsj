import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.webservice.verification.WSResponseManager as WSResponseManager
import groovy.json.JsonSlurper as JsonSlurper

RE = WS.sendRequest(findTestObject('HR/get-cookie', [('uid') : uid]))

WS.verifyElementPropertyValue(RE, 'd', 'SUCCESS')

def cook = getCookie(RE)

RS = WS.sendRequest(findTestObject('HR/jx', [('cook') : cook]))

WS.verifyElementPropertyValue(RS, 'd.userName', '曹东' /*

    out.write(cok)

    out.flush()

    out.close()
    */ ) /*File file = new File('E://TestTools/KatalonDATA/TestAPI/Data Files/scook.csv')

    BufferedWriter out = new BufferedWriter(new FileWriter(file))*/

String getCookie(ResponseObject RE) {
    def rep = RE.headerFields

    def cok = rep.get('Set-Cookie')[0].split(';')[0]

    println('-----------------------------------------------------------')

    println(cok)

    println('-----------------------------------------------------------')

    return cok
}

