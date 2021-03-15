import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import groovy.json.JsonSlurper as JsonSlurper

response = WS.sendRequestAndVerify(findTestObject('DataGlobal', [('sortBy') : 'Country Id', ('countryId') : 1]), FailureHandling.STOP_ON_FAILURE)