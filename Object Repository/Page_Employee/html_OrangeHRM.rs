<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_OrangeHRM</name>
   <tag></tag>
   <elementGuidId>80ddc4cb-56c5-4a2e-834c-05a0936c6130</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xmlns</name>
      <type>Main</type>
      <value>http://www.w3.org/1999/xhtml</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xml:lang</name>
      <type>Main</type>
      <value>en</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>lang</name>
      <type>Main</type>
      <value>en</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

        OrangeHRM
        
        
                
        
        
        
        
        
        
        
        
        
        
        
        











   
        
        
                



    
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-elementInfoDiv {color: lightblue; padding: 5px}
    
      
        
            
            
                
                
                Welcome Admin
                
                    
                        
    About

    
        
            ×
            About
        
        
            
                
                    
                        
                            Company Name: chougule mm
                        
                        
                            Version: Orangehrm OS 3.3.1
                        
                        
                            Active Employees: 93
                        
                        
                            Employees Terminated: 0
                        
                    
                
                
                        
    



    jQuery(document).ready(function() {
        
               
        jQuery('#aboutDisplayLink').click(function(event) {
            event.stopImmediatePropagation();
            jQuery('#messageToDisplayAbout').css(
                    'display', 'none');
            jQuery('#displayAbout').modal();
            jQuery('#help-menu.panelContainer').attr('style', 'display:block');
            
            var test = jQuery('.panelContainer');
            jQuery('#help-menu.panelContainer').css(
                    'display', 'block');
             
        });

        jQuery('#heartbeatSubmitBtn').click(function(event) {
            event.stopImmediatePropagation();
            jQuery(this).closest('form').ajaxSubmit(function() {
                jQuery('#messageToDisplayAbout').html('Saved');

                if (jQuery('#register_registration').is(':checked')) {
                    jQuery('#registration-section').css(
                            'display', 'none');
                }
                jQuery('#displayAbout').modal('hide');
                jQuery('#messageToDisplayAbout').css(
                        'display', 'block');
                jQuery('#welcome-menu').css('display','none');
            });
        });

        jQuery('#displayAbout').click(function(event) {
            event.stopImmediatePropagation();
        });
        
        jQuery('#heartbeatCancelBtn').click(function(event) {
            event.stopImmediatePropagation();
             jQuery('#welcome-menu').css('display','none');
                 jQuery('#displayAbout').modal('hide');
        });

    })




                        Logout
                    
                
                  
                   
            
            

    
    
        
                    
        Admin
            
            
                        
                    
                                        
                        User Management
                        
                                                
                            
                                
                                                                
                                    Users
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Job
                        
                                                
                            
                                
                                                                
                                    Job Titles
                                
                                                                
                                    Pay Grades
                                
                                                                
                                    Employment Status
                                
                                                                
                                    Job Categories
                                
                                                                
                                    Work Shifts
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Organization
                        
                                                
                            
                                
                                                                
                                    General Information
                                
                                                                
                                    Locations
                                
                                                                
                                    Structure
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Qualifications
                        
                                                
                            
                                
                                                                
                                    Skills
                                
                                                                
                                    Education
                                
                                                                
                                    Licenses
                                
                                                                
                                    Languages
                                
                                                                
                                    Memberships
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Nationalities
                        
                                                    
                           
                    
                                        
                        Configuration
                        
                                                
                            
                                
                                                                
                                    Email Configuration
                                
                                                                
                                    Email Subscriptions
                                
                                                                
                                    Localization
                                
                                                                
                                    Modules
                                
                                                                
                                    Social Media Authentication
                                
                                                                
                             
                            
                                                    
                           
                    
                                                
                                         
            
            
                    
        PIM
            
            
                        
                    
                                        
                        Configuration
                        
                                                
                            
                                
                                                                
                                    Optional Fields
                                
                                                                
                                    Custom Fields
                                
                                                                
                                    Data Import
                                
                                                                
                                    Reporting Methods
                                
                                                                
                                    Termination Reasons
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Employee List
                        
                                                    
                           
                    
                                        
                        Add Employee
                        
                                                    
                           
                    
                                        
                        Reports
                        
                                                    
                           
                    
                                                
                                         
            
            
                    
        Leave
            
            
                        
                    
                                        
                        Entitlements
                        
                                                
                            
                                
                                                                
                                    Add Entitlements
                                
                                                                
                                    Employee Entitlements
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Reports
                        
                                                
                            
                                
                                                                
                                    Leave Entitlements and Usage Report
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Configure
                        
                                                
                            
                                
                                                                
                                    Leave Period
                                
                                                                
                                    Leave Types
                                
                                                                
                                    Work Week
                                
                                                                
                                    Holidays
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Leave List
                        
                                                    
                           
                    
                                        
                        Assign Leave
                        
                                                    
                           
                    
                                                
                                         
            
            
                    
        Time
            
            
                        
                    
                                        
                        Timesheets
                        
                                                
                            
                                
                                                                
                                    Employee Timesheets
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Attendance
                        
                                                
                            
                                
                                                                
                                    Employee Records
                                
                                                                
                                    Configuration
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Reports
                        
                                                
                            
                                
                                                                
                                    Project Reports
                                
                                                                
                                    Employee Reports
                                
                                                                
                                    Attendance Summary
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Project Info
                        
                                                
                            
                                
                                                                
                                    Customers
                                
                                                                
                                    Projects
                                
                                                                
                             
                            
                                                    
                           
                    
                                                
                                         
            
            
                    
        Recruitment
            
            
                        
                    
                                        
                        Candidates
                        
                                                    
                           
                    
                                        
                        Vacancies
                        
                                                    
                           
                    
                                                
                                         
            
            
                    
        Performance
            
            
                        
                    
                                        
                        Configure
                        
                                                
                            
                                
                                                                
                                    KPIs
                                
                                                                
                                    Trackers
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Manage Reviews
                        
                                                
                            
                                
                                                                
                                    Manage Reviews
                                
                                                                
                             
                            
                                                    
                           
                    
                                        
                        Employee Trackers
                        
                                                    
                           
                    
                                                
                                         
            
            
                    
        Dashboard
            
            
                                    
                        
                            
                                         
            
            
                    
        Directory
            
            
                                    
                        
                            
                                         
            
            
                    
     
    
 
            

                  





    
        Add Employee
    

    
        


        
        
            
                
                    Full Name* First Name
 

Middle Name
 

* Last Name
 


Employee Id
  

Photograph
  Accepts jpg, .png, .gif up to 1MB. Recommended dimensions: 200px X 200px

Create Login Details
  

User Name *
 

Password *
 

Confirm Password *
 

Status *
 
Enabled
Disabled



                    
                        * Required field                    
                
                
                    
                
            
        
    

    
     


    //&lt;![CDATA[
    //we write javascript related stuff here, but if the logic gets lengthy should use a seperate js file
    var edit = &quot;Edit&quot;;
    var save = &quot;Save&quot;;
    var lang_firstNameRequired = 'Required';
    var lang_lastNameRequired = 'Required';
    var lang_userNameRequired = &quot;Should have at least 5 characters&quot;;
    var lang_passwordRequired = &quot;Should have at least 4 characters&quot;;
    var lang_unMatchingPassword = &quot;Passwords do not match&quot;;
    var lang_statusRequired = &quot;Required&quot;;
    var lang_locationRequired = &quot;Required&quot;;
    var cancelNavigateUrl = &quot;/../../index.php?menu_no_top=hr&quot;;
    var createUserAccount = &quot;0&quot;;
    var ldapInstalled = 'false';
    var fieldHelpBottom = &quot;Accepts jpg, .png, .gif up to 1MB. Recommended dimensions: 200px X 200px&quot;;
    var openIdEnabled = &quot;on&quot;;
    //]]>


             
          
         
        
        
            OrangeHRM 3.3.1
© 2005 - 2018 OrangeHRM, Inc. All rights reserved.
                 
        
        
 
        

            $(document).ready(function() {                            
                
                /* Enabling tooltips */
                $(&quot;.tiptip&quot;).tipTip();

                /* Toggling header menus */
                $(&quot;#welcome&quot;).click(function () {
                    $(&quot;#welcome-menu&quot;).slideToggle(&quot;fast&quot;);
                    $(this).toggleClass(&quot;activated-welcome&quot;);
                    return false;
                });
                
                $(&quot;#help&quot;).click(function () {
                    $(&quot;#help-menu&quot;).slideToggle(&quot;fast&quot;);
                    $(this).toggleClass(&quot;activated-help&quot;);
                    return false;
                });
                
                $('.panelTrigger').outside('click', function() {
                    $('.panelContainer').stop(true, true).slideUp('fast');
                });                

                /* 
                 * Button hovering effects 
                 * Note: we are not using pure css using :hover because :hover applies to even disabled elements.
                 * The pseudo class :enabled is not supported in IE &lt; 9.
                 */                
                $(document).on({
                    mouseenter: function () {
                        $(this).addClass('hover');                        
                    },
                    mouseleave: function () {
                        $(this).removeClass('hover');                        
                    }

                }, 'input[type=button], input[type=submit], input[type=reset]'); 
  
                /* Fading out main messages */
                $(document).on({
                    click: function() {
                        $(this).parent('div.message').fadeOut(&quot;slow&quot;);
                    }
                }, '.message a.messageCloseButton');                

                /* Toggling search form: Begins */
                //$(&quot;.toggableForm .inner&quot;).hide(); // Disabling this makes search forms to be expanded by default.

                $(&quot;.toggableForm .toggle&quot;).click(function () {
                    $(&quot;.toggableForm .inner&quot;).slideToggle('slow', function() {
                        if($(this).is(':hidden')) {
                            $('.toggableForm .tiptip').tipTip({content:'Expand for Options'});
                        } else {
                            $('.toggableForm .tiptip').tipTip({content:'Hide Options'});
                        }
                    });
                    $(this).toggleClass(&quot;activated&quot;);
                });
                /* Toggling search form: Ends */

                /* Enabling/disabling form fields: Begin */
                
                $('form.clickToEditForm input, form.clickToEditForm select, form.clickToEditForm textarea').attr('disabled', 'disabled');
                $('form.clickToEditForm input.calendar').datepicker('disable');
                $('form.clickToEditForm input[type=button]').removeAttr('disabled');
                
                $('form input.editButton').click(function(){
                    $('form.clickToEditForm input, form.clickToEditForm select, form.clickToEditForm textarea').removeAttr('disabled');
                    $('form.clickToEditForm input.calendar').datepicker('enable');
                });
                
                /* Enabling/disabling form fields: End */
                
            });
            
                

    
    


/html[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
</WebElementEntity>
