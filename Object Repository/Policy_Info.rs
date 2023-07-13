<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Policy_Info</name>
   <tag></tag>
   <elementGuidId>1c328fce-0875-4ce9-9386-15b3cc99d681</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://se2fastwebqa.sbl.com/UnifiedWeb/api/PolicyInfo?ComCode=${ComCode}&amp;PolicyNumber=${PolicyNumber}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'SB'</defaultValue>
      <description></description>
      <id>d41bd2f3-3063-4d1e-97d1-57a72272a086</id>
      <masked>false</masked>
      <name>ComCode</name>
   </variables>
   <variables>
      <defaultValue>'ULP001693'</defaultValue>
      <description></description>
      <id>7886fedd-7c28-4024-b7d9-3aeef870042a</id>
      <masked>false</masked>
      <name>PolicyNumber</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'PolicyNumber', &quot;ULP001693&quot;)
WS.verifyElementPropertyValue(response, 'TaxCategory', null)
WS.verifyElementPropertyValue(response, 'CashValue', 79349.06)
WS.verifyElementPropertyValue(response, 'FaceAmount', 500000.0)
WS.verifyElementPropertyValue(response, 'IssueDate', &quot;2021-03-20T00:00:00&quot;)
WS.verifyElementPropertyValue(response, 'EffectiveDate', &quot;2021-03-20T00:00:00&quot;)
WS.verifyElementPropertyValue(response, 'ApplicationDate', &quot;2021-03-20T00:00:00&quot;)
WS.verifyElementPropertyValue(response, 'MecStatus', &quot;00000000-0000-0000-0000-000000000002&quot;)
WS.verifyElementPropertyValue(response, 'MecStatusDesc', &quot;MEC&quot;)
WS.verifyElementPropertyValue(response, 'MaturityExpiryDate', &quot;2106-03-20T00:00:00&quot;)
WS.verifyElementPropertyValue(response, 'RenewalDate', null)
WS.verifyElementPropertyValue(response, 'Status', &quot;00000000-0000-0000-0000-000000000004&quot;)
WS.verifyElementPropertyValue(response, 'StatusDesc', &quot;Active&quot;)
WS.verifyElementPropertyValue(response, 'IssueState', &quot;RI&quot;)
WS.verifyElementPropertyValue(response, 'PlanDescription', &quot;SB UL Premium Match&quot;)
WS.verifyElementPropertyValue(response, 'ProductCode', &quot;UL&quot;)
WS.verifyElementPropertyValue(response, 'ProductLine', &quot;Universal Life&quot;)
WS.verifyElementPropertyValue(response, 'LengthOfTerm', null)
WS.verifyElementPropertyValue(response, 'VitalityStatus', null)
WS.verifyElementPropertyValue(response, 'LastUpdated', &quot;2023-06-20T22:28:01.32&quot;)
WS.verifyElementPropertyValue(response, 'Owners[0].PersonType', &quot;00000000-0000-0000-0000-000000000001&quot;)
WS.verifyElementPropertyValue(response, 'Owners[0].PersonTypeDesc', &quot;Person&quot;)
WS.verifyElementPropertyValue(response, 'Owners[0].FullName', &quot;Samyuktha TeptyB039F5&quot;)
WS.verifyElementPropertyValue(response, 'Owners[0].FirstName', &quot;Samyuktha&quot;)
WS.verifyElementPropertyValue(response, 'Owners[0].MiddleName', null)
WS.verifyElementPropertyValue(response, 'Owners[0].LastName', &quot;TeptyB039F5&quot;)
WS.verifyElementPropertyValue(response, 'Owners[0].Suffix', null)
WS.verifyElementPropertyValue(response, 'Owners[0].OrgName', &quot;Samyuktha TeptyB039F5&quot;)
WS.verifyElementPropertyValue(response, 'Owners[0].PrefComm', &quot;E-Mail&quot;)
WS.verifyElementPropertyValue(response, 'Owners[0].CustomerID', &quot;SBLULP001693&quot;)
WS.verifyElementPropertyValue(response, 'TokenNumber', null)
WS.verifyElementPropertyValue(response, 'AccountValue', 79349.06)
WS.verifyElementPropertyValue(response, 'AccountSurrenderValue', 79349.06)
WS.verifyElementPropertyValue(response, 'PackageAcknowledgement', &quot;NA&quot;)
WS.verifyElementPropertyValue(response, 'PackageAcknowledgementDate', null)
WS.verifyElementPropertyValue(response, 'MatchLifetimeLimit', 1750.0)
WS.verifyElementPropertyValue(response, 'FormNumber', &quot;SBFIXUL1&quot;)
WS.verifyElementPropertyValue(response, 'PlanCode', &quot;SBFIXUL1&quot;)
WS.verifyElementPropertyValue(response, 'FreeLook', false)





WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)
WS.verifyElementPropertyValue(response, 'CashValue', 79356.73)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
