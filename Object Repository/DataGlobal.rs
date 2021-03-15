<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>DataGlobal</name>
   <tag></tag>
   <elementGuidId>c5197256-f294-4ee0-8617-35b1d43a9d49</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.kawalcorona.com/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9ba19a5b-bb25-443d-add4-d4fcc1a56151</id>
      <masked>false</masked>
      <name>sortBy</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e361a440-b153-48bc-8318-4156a73dca2b</id>
      <masked>false</masked>
      <name>countryName</name>
   </variables>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>f854999e-8fd4-43cb-b826-f24c6a988050</id>
      <masked>false</masked>
      <name>countryId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import java.text.DecimalFormat

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

def sortBy = request.getVariables().get(&quot;sortBy&quot;)

def countryName = request.getVariables().get(&quot;countryName&quot;)

def countryId = request.getVariables().get(&quot;countryId&quot;)

def resultResponse

JsonSlurper js = new JsonSlurper()

def result = js.parseText(response.getResponseBodyContent())

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

resultSortById = result.sort({ def a, def b ->
	a.attributes.OBJECTID &lt;=> b.attributes.OBJECTID
})

switch(sortBy){
	case &quot;Country Name&quot;:
		resultResponse = (resultSortById.findAll({
			it.attributes.Country_Region == countryName
		}))
		break
	case &quot;Country Id&quot;:
		resultResponse = (resultSortById.findAll({
			it.attributes.OBJECTID == countryId
		}))
		break
	default:
		KeywordUtil.logInfo(&quot;Keyword salah.&quot;)
		break
}

if(resultResponse != []) {
	DecimalFormat df = new DecimalFormat(&quot;###,##0&quot;)

	def countryRegion = resultResponse.attributes.Country_Region.toString().replaceAll(&quot;\\[|\\]&quot;, &quot;&quot;)

	def confirmedCases = resultResponse.attributes.Confirmed.toString().replaceAll(&quot;\\[|\\]&quot;, &quot;&quot;)

	def deathCases =  resultResponse.attributes.Deaths.toString().replaceAll(&quot;\\[|\\]&quot;, &quot;&quot;)

	def recoveredCases = resultResponse.attributes.Recovered.toString().replaceAll(&quot;\\[|\\]&quot;, &quot;&quot;)

	def activeCases = resultResponse.attributes.Active.toString().replaceAll(&quot;\\[|\\]&quot;, &quot;&quot;)

	confirmedCases == &quot;null&quot; ? confirmedCases = &quot;0&quot; : false

	deathCases == &quot;null&quot; ? deathCases = &quot;0&quot; : false

	recoveredCases == &quot;null&quot; ? recoveredCases = &quot;0&quot; : false

	activeCases == &quot;null&quot; ? activeCases = &quot;0&quot; : false

	KeywordUtil.logInfo(&quot;\nNama Negara: $countryRegion&quot;+
			&quot;\nKasus Terkonfirmasi: &quot; + df.format(Double.parseDouble(confirmedCases)) +
			&quot;\nMeninggal Dunia: &quot; + df.format(Double.parseDouble(deathCases)) +
			&quot; \nSembuh: &quot; + df.format(Double.parseDouble(recoveredCases)) +
			&quot; \nKasus Aktif: &quot; + df.format(Double.parseDouble(activeCases))
			)
} else {
	KeywordUtil.logInfo(&quot;Data Tidak Ditemukan.&quot;)
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
