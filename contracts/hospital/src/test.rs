#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};
use soroban_sdk::testutils::{Address as _};

fn setup () -> (HospitalContractClient<'static>, Address, Env, Address) {
    let env = Env::default();
    let admin = Address::generate(&env);
    let contract_id: Address = env.register(HospitalContract, ());
    let client: HospitalContractClient<'_> = HospitalContractClient::new(&env, &contract_id);

    let result = client.initialize(&admin);
    (client, admin, env, result)
}

#[test]
fn test_hospital_contract() {

  let (client, admin, env, result) = setup();

    assert_eq!(result, admin);

    // test patient registration 
    env.mock_all_auths();

    let allergies = vec![&env, String::from_str(&env, "Penicillin")];
    let patient_id = client.register_patient(
        &String::from_str(&env,"Ayo"), 
        &19800101,
        &String::from_str(&env, "A+"), 
        &allergies, 
        &String::from_str(&env, "INS123YP7")
        );

        assert_eq!(patient_id, 1);

        // Test retreieving the patient data 
    
    let patient = client.get_patient(&patient_id);
    assert_eq!(patient.name, String::from_str(&env, "Ayo"));
    assert_eq!(patient.active, true);

    let updated_allergies = vec![
        &env, 
        String::from_str(&env, "Penicillin"),
        String::from_str(&env, "Peanuts")
    ];

    let updated_patient = client.update_patient(
        &patient_id,
        &String::from_str(&env,"Ayo"), 
        &19800101,
        &String::from_str(&env, "A+"), 
        &updated_allergies, 
        &String::from_str(&env, "INS123YP7-update")
        );

        assert_eq!(updated_patient.allergies.len(), 2);
        assert_eq!(updated_patient.insurance_id, String::from_str(&env, "INS123YP7-update"));


    // test doctor 
    let doctor_id = client.register_doctor(
        &String::from_str(&env, "Dr. Beulah"), 
        &String::from_str(&env, "Cardiology"), 
        &String::from_str(&env, "DOC789")
    );

    assert_eq!(doctor_id, 1);

    // Test retrieving doctor 
    let doctor = client.get_doctor(&doctor_id);
    assert_eq!(doctor.name, String::from_str(&env, "Dr. Beulah"));
    assert_eq!(doctor.active, true);

    // Test recording medical test 
    let test_date = env.ledger().timestamp();

    let test_id = client.record_medical_test(
        &patient_id, 
        &doctor_id, 
        &String::from_str(&env, "Blood pressure"), 
        &test_date, 
        &String::from_str(&env, "120/80, Normal"), 
        &String::from_str(&env, "Patient should continue his medication")
    );

    assert_eq!(test_id, 1);

    // Test retrieve the medical test 

    let test = client.get_medical_test(&test_id);
    assert_eq!(test.patient_id, patient_id);
    assert_eq!(test.doctor_id, doctor_id);

    // Test listing patients 
    let patients = client.list_patients();
    assert_eq!(patients.len(), 1);

    // Test listing doctors 
    let doctors = client.list_doctors();
    assert_eq!(doctors.len(), 1);

    // Test getting the patient tests 
    let patient_test = client.get_patients_tests(&patient_id);
    assert_eq!(patient_test.len(), 1);
    assert_eq!(patient_test.get(0).unwrap().id, test_id);


    // Test patient setting to be inactive 
    let inactive_patient = client.set_patient_active(&patient_id, &false);
    assert_eq!(inactive_patient.active, false);

    // Test setting doctor inactive 
    let inactive_doctor = client.set_doctor_active(&doctor_id, &false);
    assert_eq!(inactive_doctor.active, false);

}


#[test]
#[should_panic(expected = "Patient is inactive")]
fn test_inactive_patient() {
    let (client, _, env,_) = setup();

    env.mock_all_auths();
    // Registering the patients 
    let allergies = vec![&env, String::from_str(&env, "Penicillin")];
    let patient_id = client.register_patient(
        &String::from_str(&env,"Ayo"), 
        &19800101,
        &String::from_str(&env, "A+"), 
        &allergies, 
        &String::from_str(&env, "INS123YP7")
        );


    // Registering the doctor 

    let doctor_id = client.register_doctor(
        &String::from_str(&env, "Dr. Beulah"), 
        &String::from_str(&env, "Cardiology"), 
        &String::from_str(&env, "DOC789")
    );


    // Set patient to inactive 
    client.set_patient_active(&patient_id, &false);

    // Try to record 
    let test_date = env.ledger().timestamp();
    client.record_medical_test(
        &patient_id, 
        &doctor_id, 
        &String::from_str(&env, "Blood pressure"), 
        &test_date, 
        &String::from_str(&env, "120/80, Normal"), 
        &String::from_str(&env, "Patient should continue his medication")
    );


}

// Quick class work
// Test doctor is inactive 
// Test for contract already initialized 
// patient not registered
