package fr.myapplication.adapter;

import android.net.Uri
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ImageView
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import com.bumptech.glide.Glide
import fr.myapplication.DisplayMetricsHelper
import fr.myapplication.R
import fr.myapplication.ReadActivity
import fr.myapplication.ScanModel

class ReadAdapter(
    val context: ReadActivity,
    private val scan: ScanModel?,
    private val layoutId: Int,
    val chapitre: Int
) : RecyclerView.Adapter<ReadAdapter.ViewHolder>() {

    class ViewHolder(view: View) : RecyclerView.ViewHolder(view) {
        val readImage = view.findViewById<ImageView>(R.id.image_item)
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val view = LayoutInflater
            .from(parent.context)
            .inflate(layoutId, parent, false)
        return ViewHolder(view)
    }

    override fun getItemCount(): Int {
        return (scan?.img_epi?.get("$chapitre") ?: 0)+1
    }

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        val p1 = position + 1
        println("-------------$p1---$chapitre")

        //charger image
        Glide.with(context)
            .load(Uri.parse("android.resource://fr.myapplication/drawable/" + scan?.name + "_" + chapitre + "_" + p1))
            .into(holder.readImage)
        //holder.numbertext.text = DisplayMetricsHelper.getImageWidth(context, "android.resource://fr.myapplication/drawable/" + scan?.name + "_0" + p1 + ".png").toString()

        holder.readImage.layoutParams.width = DisplayMetricsHelper.getScreenWidth(context)
        holder.readImage.layoutParams.height = DisplayMetricsHelper.getHeightSize(
            context,
            ("android.resource://fr.myapplication/drawable/" + scan?.name + "_" + chapitre + "_" + p1)
        )
    }

}