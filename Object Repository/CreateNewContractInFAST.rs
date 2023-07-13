<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateNewContractInFAST</name>
   <tag></tag>
   <elementGuidId>cabf482b-9b3e-4098-b69c-78921883672b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>SOAPAction</name>
      <type>Main</type>
      <value>http://tempuri.org/IOrchstnAdapter/RunOrchestration</value>
      <webElementGuid>8f2bb400-96ed-4474-bc6d-e15d8b9604f6</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>a435bee4-6bcd-4a30-b815-c2ed1ec5f141</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot;?>
&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tem=&quot;http://tempuri.org/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;tem:RunOrchestration>
         &lt;!--Optional:-->
         &lt;tem:orchName>JetIssueDataAggForACORD103&lt;/tem:orchName>
         &lt;!--Optional:-->
         &lt;tem:inputXML>&lt;![CDATA[&lt;TXLife xmlns=&quot;http://ACORD.org/Standards/Life/2&quot;>
	&lt;TXLifeRequest PrimaryObjectID=&quot;Holding_1&quot;>
		&lt;TransRefGUID>AU22032524QM9HQQPG&lt;/TransRefGUID>
		&lt;TransType tc=&quot;103&quot;>New Business Submission&lt;/TransType>
		&lt;TransSubType tc=&quot;10301&quot;>New Business Submission - Life&lt;/TransSubType>
		&lt;TransExeDate>2023-01-23&lt;/TransExeDate>
		&lt;TransExeTime>12:56:50&lt;/TransExeTime>
		&lt;OLifE>
			&lt;SourceInfo>
				&lt;SourceInfoName>ESB&lt;/SourceInfoName>
				&lt;SourceInfoDescription>FAST Deliver&lt;/SourceInfoDescription>
			&lt;/SourceInfo>
			&lt;Holding id=&quot;Holding_1&quot;>
				&lt;HoldingTypeCode tc=&quot;2&quot;>Policy&lt;/HoldingTypeCode>
				&lt;Policy>
					&lt;PolNumber>AU22001859&lt;/PolNumber>
					&lt;LineOfBusiness tc=&quot;1&quot;>Life&lt;/LineOfBusiness>
					&lt;ProductType tc=&quot;3&quot;>Universal Life&lt;/ProductType>
					&lt;ProductCode>SBFIXUL1&lt;/ProductCode>
					&lt;ProductVersionCode>2021.10.15&lt;/ProductVersionCode>
					&lt;CarrierCode>SB&lt;/CarrierCode>
					&lt;PlanName>SB UL PREMIUM MATCH&lt;/PlanName>
					&lt;ShortName>SB UL PREMIUM MATCH&lt;/ShortName>
					&lt;PolicyStatus tc=&quot;25&quot;>Issued&lt;/PolicyStatus>
					&lt;IssueType tc=&quot;1&quot;>FULL_UNDERWRITING&lt;/IssueType>
					&lt;Jurisdiction tc=&quot;12&quot;>FL&lt;/Jurisdiction>
					&lt;Life>
						&lt;FaceAmt>500000.0&lt;/FaceAmt>
						&lt;Coverage id=&quot;Base_Coverage&quot;>
							&lt;IndicatorCode tc=&quot;1&quot;>Base&lt;/IndicatorCode>
							&lt;DeathBenefitOptType tc=&quot;1&quot;>Level&lt;/DeathBenefitOptType>
							&lt;ModalPremAmt>367.00&lt;/ModalPremAmt>
							&lt;EffDate>2023-01-23&lt;/EffDate>
							&lt;PayToYear>20&lt;/PayToYear>
							&lt;LifeParticipant id=&quot;LP_Insured_1&quot; PartyID=&quot;Party_PI_1&quot;>
								&lt;LifeParticipantRoleCode tc=&quot;1&quot;>Insured&lt;/LifeParticipantRoleCode>
								&lt;TobaccoPremiumBasis tc=&quot;1&quot;>Non-Smoker&lt;/TobaccoPremiumBasis>
								&lt;TempFlatExtraAmt>0&lt;/TempFlatExtraAmt>
								&lt;UnderwritingClass tc=&quot;10&quot;>Preferred Non-Tobacco&lt;/UnderwritingClass>
								&lt;UnderwritingStatus tc=&quot;6&quot;>Approved&lt;/UnderwritingStatus>
								&lt;TempFlatExtraDuration>0&lt;/TempFlatExtraDuration>
							&lt;/LifeParticipant>
							&lt;LifeParticipant id=&quot;LP_Primary_Bene_1&quot; PartyID=&quot;Party_PB_Primary_Bene_1&quot;>
								&lt;LifeParticipantRoleCode tc=&quot;7&quot;>Primary Bene&lt;/LifeParticipantRoleCode>
							&lt;/LifeParticipant>
						&lt;/Coverage>
						&lt;OLifEExtension VendorCode=&quot;154&quot;>
							&lt;MatchValueVersion>2021.10.15&lt;/MatchValueVersion>
							&lt;MatchPercentage>1&lt;/MatchPercentage>
						&lt;/OLifEExtension>
					&lt;/Life>
					&lt;ApplicationInfo>
						&lt;ApplicationType tc=&quot;1&quot;>New&lt;/ApplicationType>
						&lt;SignedDate>2023-01-23&lt;/SignedDate>
						&lt;MaxRiskAmt>1000000.0&lt;/MaxRiskAmt>
						&lt;SignatureInfo>
							&lt;SignatureRoleCode tc=&quot;18&quot;>Owner&lt;/SignatureRoleCode>
							&lt;SignatureDate>2023-01-23&lt;/SignatureDate>
						&lt;/SignatureInfo>
					&lt;/ApplicationInfo>
				&lt;/Policy>
				&lt;Arrangement id=&quot;Arr_1&quot;>
					&lt;ArrMode tc=&quot;4&quot;>Monthly&lt;/ArrMode>
					&lt;ArrType tc=&quot;19&quot;>Initial Payment&lt;/ArrType>
					&lt;ModalAmt>367.00&lt;/ModalAmt>
					&lt;PaymentMethod tc=&quot;7&quot;>ACH&lt;/PaymentMethod>
				&lt;/Arrangement>
				&lt;Banking>
					&lt;BankAcctType tc=&quot;2&quot;>Checking&lt;/BankAcctType>
					&lt;AccountNumber>012345678910&lt;/AccountNumber>
					&lt;RoutingNum>021200339&lt;/RoutingNum>
					&lt;BankName>BANK OF AMERICA N.A.&lt;/BankName>
				&lt;/Banking>
			&lt;/Holding>
			&lt;Party id=&quot;Party_PI_1&quot;>
				&lt;PartyTypeCode tc=&quot;1&quot;>Person&lt;/PartyTypeCode>
				&lt;FullName>Patrick J Malizio&lt;/FullName>
				&lt;GovtID>666225447&lt;/GovtID>
				&lt;GovtIDTC tc=&quot;1&quot;>Social Security Number&lt;/GovtIDTC>
				&lt;PrefComm tc=&quot;3&quot;>Email&lt;/PrefComm>
				&lt;IDReferenceNo>0017b0000100WHTAA2&lt;/IDReferenceNo>
				&lt;IDReferenceType tc=&quot;35&quot;>Customer Number&lt;/IDReferenceType>
				&lt;Person>
					&lt;FirstName>Patrick&lt;/FirstName>
					&lt;MiddleName>J&lt;/MiddleName>
					&lt;LastName>Malizio&lt;/LastName>
					&lt;Gender tc=&quot;1&quot;>Male&lt;/Gender>
					&lt;BirthDate>2005-01-22&lt;/BirthDate>
					&lt;DriversLicenseNum>M620521742270&lt;/DriversLicenseNum>
					&lt;DriversLicenseState tc=&quot;12&quot;>FL&lt;/DriversLicenseState>
					&lt;BirthCountry tc=&quot;1&quot;>US&lt;/BirthCountry>
				&lt;/Person>
				&lt;Address id=&quot;Party_PI_Address_1&quot;>
					&lt;AddressTypeCode tc=&quot;0&quot;>Unknown&lt;/AddressTypeCode>
					&lt;Line1>1210 Carvell Dr&lt;/Line1>
					&lt;City>Winter Park&lt;/City>
					&lt;AddressState>FL&lt;/AddressState>
					&lt;AddressStateTC tc=&quot;12&quot;>FL&lt;/AddressStateTC>
					&lt;Zip>32792&lt;/Zip>
					&lt;PrefAddr tc=&quot;1&quot;>true&lt;/PrefAddr>
				&lt;/Address>
				&lt;Phone id=&quot;Party_PI_Phone_1&quot;>
					&lt;PhoneTypeCode tc=&quot;12&quot;>Mobile&lt;/PhoneTypeCode>
					&lt;PhoneValue>312-818-0591&lt;/PhoneValue>
				&lt;/Phone>
				&lt;EMailAddress id=&quot;Party_PI_EmailAddress_1&quot;>
					&lt;EMailType tc=&quot;2&quot;>Personal&lt;/EMailType>
					&lt;AddrLine>yashtester012622+test3@gmail.com&lt;/AddrLine>
				&lt;/EMailAddress>
				&lt;Risk>
					&lt;ExistingInsuranceInd tc=&quot;0&quot;>False&lt;/ExistingInsuranceInd>
				&lt;/Risk>
			&lt;/Party>
			&lt;Party id=&quot;Party_PB_Primary_Bene_1&quot;>
				&lt;PartyTypeCode tc=&quot;1&quot;>Person&lt;/PartyTypeCode>
				&lt;Person>
					&lt;FirstName>Test&lt;/FirstName>
					&lt;LastName>Test&lt;/LastName>
					&lt;BirthDate>2005-01-22&lt;/BirthDate>
				&lt;/Person>
			&lt;/Party>
			&lt;Relation id=&quot;Relation_PI_Insured&quot; OriginatingObjectID=&quot;Holding_1&quot; RelatedObjectID=&quot;Party_PI_1&quot;>
				&lt;OriginatingObjectType tc=&quot;4&quot;>Holding&lt;/OriginatingObjectType>
				&lt;RelatedObjectType tc=&quot;6&quot;>Party&lt;/RelatedObjectType>
				&lt;RelationRoleCode tc=&quot;32&quot;>Insured&lt;/RelationRoleCode>
			&lt;/Relation>
			&lt;Relation id=&quot;Relation_PI_Owner&quot; OriginatingObjectID=&quot;Holding_1&quot; RelatedObjectID=&quot;Party_PI_1&quot;>
				&lt;OriginatingObjectType tc=&quot;4&quot;>Holding&lt;/OriginatingObjectType>
				&lt;RelatedObjectType tc=&quot;6&quot;>Party&lt;/RelatedObjectType>
				&lt;RelationRoleCode tc=&quot;8&quot;>Owner&lt;/RelationRoleCode>
			&lt;/Relation>
			&lt;Relation id=&quot;Relation_Holding_Payor&quot; OriginatingObjectID=&quot;Holding_1&quot; RelatedObjectID=&quot;Party_PI_1&quot;>
				&lt;OriginatingObjectType tc=&quot;4&quot;>Holding&lt;/OriginatingObjectType>
				&lt;RelatedObjectType tc=&quot;6&quot;>Party&lt;/RelatedObjectType>
				&lt;RelationRoleCode tc=&quot;31&quot;>Payor&lt;/RelationRoleCode>
			&lt;/Relation>
			&lt;Relation id=&quot;Relation_Holding_Primary_Bene_1&quot; OriginatingObjectID=&quot;Holding_1&quot; RelatedObjectID=&quot;Party_PB_Primary_Bene_1&quot;>
				&lt;OriginatingObjectType tc=&quot;4&quot;>Holding&lt;/OriginatingObjectType>
				&lt;RelatedObjectType tc=&quot;6&quot;>Party&lt;/RelatedObjectType>
				&lt;RelationRoleCode tc=&quot;34&quot;>Primary Bene&lt;/RelationRoleCode>
				&lt;InterestPercent>100&lt;/InterestPercent>
			&lt;/Relation>
			&lt;Relation id=&quot;Relation_PI_Primary_Bene_1&quot; OriginatingObjectID=&quot;Party_PI_1&quot; RelatedObjectID=&quot;Party_PB_Primary_Bene_1&quot;>
				&lt;OriginatingObjectType tc=&quot;6&quot;>Party&lt;/OriginatingObjectType>
				&lt;RelatedObjectType tc=&quot;6&quot;>Party&lt;/RelatedObjectType>
				&lt;RelationRoleCode tc=&quot;2&quot;>Child&lt;/RelationRoleCode>
			&lt;/Relation>
		&lt;/OLifE>
	&lt;/TXLifeRequest>
&lt;/TXLife>]]>&lt;/tem:inputXML>
      &lt;/tem:RunOrchestration>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://topfastext002q.sbl.com/OrchstnAdapter.svc</soapServiceEndpoint>
   <soapServiceFunction>RunOrchestration</soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
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

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress>http://topfastext002q.sbl.com/OrchstnAdapter.svc?WSDL</wsdlAddress>
</WebServiceRequestEntity>
