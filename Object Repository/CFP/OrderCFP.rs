<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>OrderCFP</name>
   <tag></tag>
   <elementGuidId>ae010454-a402-435f-a3b9-9a39211b861d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;NoKontrak\&quot;: \&quot;${nokontrak}\&quot;,\n    \&quot;CustName\&quot;: \&quot;${custname}\&quot;,\n    \&quot;HPNumber\&quot;: \&quot;${hpnumber}\&quot;,\n    \&quot;NoPolisi\&quot;: \&quot;${nopol}\&quot;,\n    \&quot;NoRangka\&quot;: \&quot;${norangka}\&quot;,\n    \&quot;CustAddress\&quot;: \&quot;${custaddress}\&quot;,\n    \&quot;NIK\&quot;: \&quot;${nik}\&quot;,\n    \&quot;NPWP\&quot;: \&quot;${npwp}\&quot;,\n    \&quot;Source\&quot;: \&quot;${source}\&quot;\n}&quot;,
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
      <webElementGuid>0ed51669-c4a5-42fe-92b3-29f546b53a78</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic YmNhZmFwcHM6QWRtaW4xMjM=</value>
      <webElementGuid>9d120144-d4df-49be-8702-772c3173f01f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://192.168.29.71:12103/CFP/OtherOrderCFP_PS</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>f667e556-8d9e-40f8-a540-e7a2dcae967e</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'9430710308001'</defaultValue>
      <description></description>
      <id>7778332e-c959-4217-a67b-0ae72184ceb1</id>
      <masked>false</masked>
      <name>nokontrak</name>
   </variables>
   <variables>
      <defaultValue>'YNNO DANRDYDO'</defaultValue>
      <description></description>
      <id>617b9d60-9473-467f-9a57-bf3312314173</id>
      <masked>false</masked>
      <name>custname</name>
   </variables>
   <variables>
      <defaultValue>'0811145284'</defaultValue>
      <description></description>
      <id>c05ab65e-393f-40bc-9307-c69ceecae302</id>
      <masked>false</masked>
      <name>hpnumber</name>
   </variables>
   <variables>
      <defaultValue>'X 4459 IOP'</defaultValue>
      <description></description>
      <id>e04cc652-14f3-4881-8b72-8554a8ee3d03</id>
      <masked>false</masked>
      <name>nopol</name>
   </variables>
   <variables>
      <defaultValue>'234JIIOE'</defaultValue>
      <description></description>
      <id>6cb026f9-4b96-47a9-bfd2-f57b8d7f4d4a</id>
      <masked>false</masked>
      <name>norangka</name>
   </variables>
   <variables>
      <defaultValue>'U10O   AN0OL  RT1N  N2L1TKE, TELUKAN, GROGOL, SUKOHARJO, JAWA TENGAH, 57552'</defaultValue>
      <description></description>
      <id>f70d9bb7-94b8-45d9-a402-c570f7569ecc</id>
      <masked>false</masked>
      <name>custaddress</name>
   </variables>
   <variables>
      <defaultValue>'3171051205770001'</defaultValue>
      <description></description>
      <id>e7ea80bd-bdcf-49bf-86f7-a7c5be317205</id>
      <masked>false</masked>
      <name>nik</name>
   </variables>
   <variables>
      <defaultValue>'SchedulerCFP'</defaultValue>
      <description></description>
      <id>342064ef-78de-4802-897c-f99320295e26</id>
      <masked>false</masked>
      <name>source</name>
   </variables>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>597d6d0f-b492-4d1c-9ad7-e1d2eecdaa49</id>
      <masked>false</masked>
      <name>npwp</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
