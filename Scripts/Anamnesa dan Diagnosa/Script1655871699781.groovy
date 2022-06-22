import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://demo.epuskesmas.id/')

WebUI.click(findTestObject('Object Repository/Page_ePuskesmas/li_Infokes Manajemen Pasien                _bf0a20'))

WebUI.click(findTestObject('Object Repository/Page_ePuskesmas/span_ePuskesmas'))

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Masuk/input_Indonesia_email'), 'willem')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_e-Puskesmas - Masuk/input_Indonesia_password'), 'iSJZYhOg0uc=')

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Masuk/button_Login'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas -/button_PUSKESMAS PUSKESMAS1'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas -/a_Pendaftaran'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas -/a_Pasien  KK'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Pasien dan Kartu Keluarga/span_175'))

WebUI.doubleClick(findTestObject('Object Repository/Page_e-Puskesmas - Pasien dan Kartu Keluarga/td_MIKELL'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Pasien - Lihat Data/a_Pendaftaran'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Pendaftaran - Buat Baru/button_Rawat Jalan'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Pendaftaran - Buat Baru/button_Umum9100'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Pendaftaran - Buat Baru/button_Simpan'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Pendaftaran - Buat Baru/a_Pelayanan'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Pendaftaran - Buat Baru/a_Medis'))

WebUI.doubleClick(findTestObject('Object Repository/Page_e-Puskesmas - Pelayanan Medis/td_Umum'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - PTM - Buat Baru/button_OK'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - PTM - Buat Baru/a_Anamnesa'))

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/input__dokter_nama'), 'a')

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/div_Ray S.Farm (Apoteker)'))

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/input__dokter_nama'), 'Ray S.Farm')

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/textarea__Anamnesakeluhan_utama'),
	'sakit')

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/div_sakit kepala'))

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/input_Thn_Anamnesalama_sakit_bulan'),
	'1')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/select_COMPOS MENTIS                       _312887'),
	'SOMNOLEN', true)

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/input__PeriksaFisiksistole'), '123')

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/input__PeriksaFisikdiastole'), '123')

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/input__PeriksaFisikdetak_nadi'),
	'23')

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/input__PeriksaFisiknafas'), '21')

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/input__PeriksaFisiktinggi'), '189')

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/input__PeriksaFisikberat'), '67')

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/input_Gawat Darurat_PeriksaFisiktriage'))

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/textarea__Anamnesaedukasi'), 'asdasd')

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/textarea__Anamnesaaskep'), 'asdas')

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/button_Simpan'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Anamnesa - Buat Baru/a_Diagnosa'))

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Diagnosa - Buat Baru/input_Status_perawat_nama'), 'as')

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Diagnosa - Buat Baru/input_Status_perawat_nama'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Diagnosa - Buat Baru/div_I GUSTI NGURAH ARTA PASCA SUPUTRA (Dokt_f68b5f'))

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Diagnosa - Buat Baru/input_Status_perawat_nama'), 'I GUSTI NGURAH ARTA PASCA SUPUTRA')

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Diagnosa - Buat Baru/input_Status_diagnosa_id'), 'as')

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Diagnosa - Buat Baru/div_A05 - Other bacterial foodborne intoxic_cb08f5'))

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Diagnosa - Buat Baru/input_Status_diagnosa_id'), 'A05')

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Diagnosa - Buat Baru/button_Simpan'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Diagnosa - Buat Baru/button_Selesai'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_e-Puskesmas - Diagnosa - Buat Baru/select_- PILIH -                           _e615b1'),
	'01', true)

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Diagnosa - Buat Baru/input__Pelayananlama_hari'), '01')

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Diagnosa - Buat Baru/button_Selesai'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Diagnosa - Buat Baru/span_PUSKESMAS PUSKESMAS1'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Diagnosa - Buat Baru/a_Keluar'))

WebUI.closeBrowser()



