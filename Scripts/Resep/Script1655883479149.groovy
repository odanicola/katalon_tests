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

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - PTM - Buat Baru/a_Resep'))

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/input__dokter_nama'), 'a')

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/div_Yandri (Staf Non Medis)'))

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/input__dokter_nama'), 'Yandri')

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/input_Perawat  Bidan  Nutrisionist  Sanitar_eb3618'), 
    'a')

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/div_DR RAFLES (Dokter Umum)'))

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/input_Perawat  Bidan  Nutrisionist  Sanitar_eb3618'), 
    'DR RAFLES')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/select_- PILIH -                           _c41755'), 
    'R3', true)

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/input_Keterangan_ResepDetail1obat_jumlah_pe_125b89'), 
    '1')

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/input_Keterangan_obat_nama'), 'a')

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/div_01006 - Amoksilin kaplet 500 mg (1205)'))

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/input_Keterangan_obat_nama'), 'Amoksilin kaplet 500 mg')

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/input_Keterangan_ResepDetail1obat_jumlah'), 
    '1')

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/input_Keterangan_signa_nama'), 's')

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/div_1X1 TETES'))

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/input_Keterangan_signa_nama'), '1X1 TETES')

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/select_- PILIH -                           _46032d'), 
    '1', true)

WebUI.setText(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/input_Keterangan_ResepDetail1obat_keterangan'), 
    '123')

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/button_Tambah'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/input_Keterangan_check_all'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/button_Simpan'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/span_PUSKESMAS PUSKESMAS1'))

WebUI.click(findTestObject('Object Repository/Page_e-Puskesmas - Resep - Buat Baru/a_Keluar'))

WebUI.closeBrowser()

