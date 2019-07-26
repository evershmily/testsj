<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>https://hr.aetest.bwae.org/kpi-salary-pc/RPC406CFAFEC8BD5BA8973308A3E1358AC9</description>
   <name>get-cookie</name>
   <tag></tag>
   <elementGuidId>2f03cf38-37fd-4205-b27e-7ca03992173f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;token\&quot;:${uid}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://hr.aetest.bwae.org/kpi-salary-pc/RPC406CFAFEC8BD5BA8973308A3E1358AC9</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>findTestData('uid').getValue(1, 1)</defaultValue>
      <description></description>
      <id>683b2c5e-18ac-4570-8070-72c94a038ebf</id>
      <masked>false</masked>
      <name>uid</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*
import groovy.json.JsonOutput

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
println(&quot;===================&quot;)
println(response.headerFields)


//WS.verifyElementPropertyValue(response, &quot;msg&quot;, &quot;200&quot;)
//def jsonrep=new JsonSlurper()
//def jsonrp=jsonrep.parseText(response.headerFields)
def rep=response.headerFields
//E:\TestTools\KatalonDATA\TestAPI\Data Files\sid.xls
//File file=new File(&quot;Data Files scook.csv&quot;)
File file=new File(&quot;E://TestTools/KatalonDATA/TestAPI/Data Files/scook.csv&quot;)

def output = JsonOutput.toJson(rep)
println(output)

BufferedWriter out=new BufferedWriter(new FileWriter(file))
/*println(&quot;===================&quot;)
println(rep.toString())
*/
def cok=rep.get(&quot;Set-Cookie&quot;)[0].split(&quot;;&quot;)[0]
println(cok)

//out.newLine()
out.write(cok)
out.flush();
out.close();

/*
assertThat(response.getHeaderFields().get('Content-Type').toString()).isEqualTo('[application/json;charset=UTF-8]')

assertThat(response.getHeaderFields().containsKey('Content-Type')).isTrue()
*/</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
