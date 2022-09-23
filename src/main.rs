// Marvish Chandra

use std::env::{args, Args}

fn calculateGDP(product1quantity,product1price,product2quantity,product2price) {
    let nominalGDP = product1quantity * product1price + product2quantity * product2price
    let realGDP = product1quantity * product2price + product2quantity * product1price

    println!("Calculated Nominal GDP Of a Given Year: {}", nominalGDP);
    println!("Calculated Real GDP Of a Given Year: {}", realGDP);

}

fn calculatingDeflation(NominalGDP,RealGDP,NewDeflation,OldDeflation){
    let priceLevel = (NominalGDP / RealGDP)
    let GDPdeflator = ((NewDeflation - OldDeflation) / OldDeflation)) * 100
}

fn unemploymentRate(unemployedPersons,workforce){
    let unemployment = unemployedPersons / workforce
}
