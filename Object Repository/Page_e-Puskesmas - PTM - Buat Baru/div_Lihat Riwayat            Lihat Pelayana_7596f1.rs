<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Lihat Riwayat            Lihat Pelayana_7596f1</name>
   <tag></tag>
   <elementGuidId>efa0c387-bb0b-4e29-a43b-aa11707b853e</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//section[@id='content']/div/div[2]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.panel-body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>078670ed-3086-44d7-aae1-931ddf863513</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>panel-body</value>
      <webElementGuid>849c92bf-1ad4-4270-90e5-add5640f9d0b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        
            Lihat Riwayat
            Lihat Pelayanan
        
        
                                                Lihat Semua
                                    Lihat RiwayatAnamnesa Diagnosa Resep Alkes Obat/Alkes Odontogram Laboratorium Tindakan MTBS Imunisasi Kartu Bayi Keur KB PKPR KIA Periksa Gizi TB Paru Periksa IMS Konseling HIV Asuhan Keperawatan Tes IVA Tumbuh Kembang Anak Caten PTM Mata Pengkajian Resiko Jatuh Prima Psikologi PAL Haji COVID-19 Diare  function showRiwayat()
                {
                    $.ajax({
                        url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayat/0000000001016925&quot;,
                        method: &quot;GET&quot;,
                        dataType: &quot;json&quot;,
                        data: {
                            _token: &quot;cfXPVbIh2JbXSJLCw8P08CFXUJHe8aAr38s4Ekin&quot;,
                            page: &quot;1&quot;
                        },
                        success: function(data) {
                            $(&quot;#modal .modal-title&quot;).html(data.title);
                            $(&quot;#modal .modal-form&quot;).html(data.form);
                            openmodal(&quot;modal-lg&quot;);
                        },
                        error: function (xhr, ajaxOptions, thrownError) {
                            alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                            console.log(xhr.status);
                            console.log(thrownError);
                        }
                    });
                }
                $(() => {
                    $(&quot;#button_index&quot;).bind(&quot;click&quot;, ({ delegateTarget }) => {
                        window.location.replace($(delegateTarget).attr(&quot;href&quot;))
                    })
                })                    
    </value>
      <webElementGuid>923afebd-da14-4de0-aa86-bae06f86de3a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;content&quot;)/div[@class=&quot;panel panel-default container-fluid&quot;]/div[@class=&quot;panel-body&quot;]</value>
      <webElementGuid>a9158b95-6ee9-42eb-bb64-dddb1ca0e509</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//section[@id='content']/div/div[2]</value>
      <webElementGuid>75ec16a8-46ea-47a5-96d7-2db7c20421f5</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Lihat Semua'])[1]/following::div[1]</value>
      <webElementGuid>cacabe3b-ddd3-47d1-aabf-01cbf258ffa9</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Buat Baru Skrining PTM'])[1]/following::div[1]</value>
      <webElementGuid>c76ab510-e54b-4345-9923-29a4411f126d</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//section/div/div[2]</value>
      <webElementGuid>dbed030d-9868-47b8-b6ec-7e7f2ac883ff</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = '
        
            Lihat Riwayat
            Lihat Pelayanan
        
        
                                                Lihat Semua
                                    Lihat RiwayatAnamnesa Diagnosa Resep Alkes Obat/Alkes Odontogram Laboratorium Tindakan MTBS Imunisasi Kartu Bayi Keur KB PKPR KIA Periksa Gizi TB Paru Periksa IMS Konseling HIV Asuhan Keperawatan Tes IVA Tumbuh Kembang Anak Caten PTM Mata Pengkajian Resiko Jatuh Prima Psikologi PAL Haji COVID-19 Diare  function showRiwayat()
                {
                    $.ajax({
                        url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayat/0000000001016925&quot;,
                        method: &quot;GET&quot;,
                        dataType: &quot;json&quot;,
                        data: {
                            _token: &quot;cfXPVbIh2JbXSJLCw8P08CFXUJHe8aAr38s4Ekin&quot;,
                            page: &quot;1&quot;
                        },
                        success: function(data) {
                            $(&quot;#modal .modal-title&quot;).html(data.title);
                            $(&quot;#modal .modal-form&quot;).html(data.form);
                            openmodal(&quot;modal-lg&quot;);
                        },
                        error: function (xhr, ajaxOptions, thrownError) {
                            alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                            console.log(xhr.status);
                            console.log(thrownError);
                        }
                    });
                }
                $(() => {
                    $(&quot;#button_index&quot;).bind(&quot;click&quot;, ({ delegateTarget }) => {
                        window.location.replace($(delegateTarget).attr(&quot;href&quot;))
                    })
                })                    
    ' or . = '
        
            Lihat Riwayat
            Lihat Pelayanan
        
        
                                                Lihat Semua
                                    Lihat RiwayatAnamnesa Diagnosa Resep Alkes Obat/Alkes Odontogram Laboratorium Tindakan MTBS Imunisasi Kartu Bayi Keur KB PKPR KIA Periksa Gizi TB Paru Periksa IMS Konseling HIV Asuhan Keperawatan Tes IVA Tumbuh Kembang Anak Caten PTM Mata Pengkajian Resiko Jatuh Prima Psikologi PAL Haji COVID-19 Diare  function showRiwayat()
                {
                    $.ajax({
                        url: &quot;https://demo.epuskesmas.id/pelayanan/showriwayat/0000000001016925&quot;,
                        method: &quot;GET&quot;,
                        dataType: &quot;json&quot;,
                        data: {
                            _token: &quot;cfXPVbIh2JbXSJLCw8P08CFXUJHe8aAr38s4Ekin&quot;,
                            page: &quot;1&quot;
                        },
                        success: function(data) {
                            $(&quot;#modal .modal-title&quot;).html(data.title);
                            $(&quot;#modal .modal-form&quot;).html(data.form);
                            openmodal(&quot;modal-lg&quot;);
                        },
                        error: function (xhr, ajaxOptions, thrownError) {
                            alert(&quot;Terjadi kesalahan sistem, silahkan hubungi team support kami!&quot;, &quot;danger&quot;);
                            console.log(xhr.status);
                            console.log(thrownError);
                        }
                    });
                }
                $(() => {
                    $(&quot;#button_index&quot;).bind(&quot;click&quot;, ({ delegateTarget }) => {
                        window.location.replace($(delegateTarget).attr(&quot;href&quot;))
                    })
                })                    
    ')]</value>
      <webElementGuid>0a2c4664-1ce4-45d0-a414-db93b710fcd4</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
